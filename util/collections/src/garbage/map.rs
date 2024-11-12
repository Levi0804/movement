use crate::garbage::Duration;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub struct GcMap<K, V>
where
	K: Eq + Hash,
	V: Eq + Hash,
{
	/// The number of milliseconds a value is valid for.
	value_ttl_ms: Duration,
	/// The duration of a garbage collection slot in milliseconds.
	/// This is used to bin values into slots for O(value_ttl_ms/gc_slot_duration_ms * log value_ttl_ms/gc_slot_duration_ms) garbage collection.
	gc_slot_duration_ms: Duration,
	/// The value lifetimes, indexed by slot.
	value_lifetimes: BTreeMap<u64, HashMap<K, V>>,
}

impl<K, V> GcMap<K, V>
where
	K: Eq + Hash,
	V: Eq + Hash,
{
	/// Creates a new GcMap with a specified garbage collection slot duration.
	pub fn new(value_ttl_ms: Duration, gc_slot_duration_ms: Duration) -> Self {
		GcMap { value_ttl_ms, gc_slot_duration_ms, value_lifetimes: BTreeMap::new() }
	}

	/// Gets a value for a key
	pub fn get_value(&self, key: &K) -> Option<&V> {
		// check each slot for the key
		for lifetimes in self.value_lifetimes.values().rev() {
			// reverse order is better average case because highly-used values will be moved up more often
			match lifetimes.get(key) {
				Some(value) => {
					// check if the value is still valid
					return Some(value);
				}
				None => {}
			}
		}

		None
	}

	/// Removes the value for an key.
	pub fn remove_value(&mut self, key: &K) {
		// check each slot for the key
		for lifetimes in self.value_lifetimes.values_mut().rev() {
			if lifetimes.remove(key).is_some() {
				break;
			}
		}
	}

	/// Sets the value for for a key
	pub fn set_value(&mut self, key: K, value: V, current_time_ms: u64) {
		// remove the old key
		self.remove_value(&key);

		// compute the slot for the new lifetime and add accordingly
		let slot = current_time_ms / self.gc_slot_duration_ms.get();

		// add the new value
		self.value_lifetimes.entry(slot).or_insert_with(HashMap::new).insert(key, value);
	}

	/// Garbage collects values that have expired.
	/// This should be called periodically.
	pub fn gc(&mut self, current_time_ms: u64) {
		let gc_slot = current_time_ms / self.gc_slot_duration_ms.get();

		// remove all slots that are too old
		let slot_cutoff = gc_slot - self.value_ttl_ms.get() / self.gc_slot_duration_ms.get();
		let slots_to_remove: Vec<u64> = self
			.value_lifetimes
			.keys()
			.take_while(|slot| **slot < slot_cutoff)
			.cloned()
			.collect();
		for slot in slots_to_remove {
			self.value_lifetimes.remove(&slot);
		}
	}
}
