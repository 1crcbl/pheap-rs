#[cfg(test)]
use super::PairingHeap;

#[cfg(test)]
fn create_heap(start: i32, end: i32) -> PairingHeap<i32, i32> {
    let mut ph = PairingHeap::<i32, i32>::new();
    for ii in start..end {
        ph.insert(ii, ii);
    }
    ph
}

#[test]
fn create_insert() {
    let mut ph = PairingHeap::<i32, i32>::new();
    assert_eq!(0, ph.len());
    assert!(ph.is_empty());

    for ii in 1..=10 {
        ph.insert(ii, ii);
    }

    assert_eq!(10, ph.len());
}

#[test]
fn find_min() {
    let ph = create_heap(0, 0);
    assert!(ph.find_min().is_none());
    assert!(ph.find_min_with_prio().is_none());

    let ph = create_heap(1, 11);
    let min = ph.find_min();
    assert!(min.is_some());
    assert_eq!(1, min.unwrap());

    let min_prio = ph.find_min_with_prio();
    assert!(min_prio.is_some());
    let (k, p) = min_prio.unwrap();
    assert_eq!(1, k);
    assert_eq!(1, p);
}

#[test]
fn merge() {
    let ph1 = create_heap(1, 11);
    let len1 = ph1.len();
    let ph2 = create_heap(11, 21);
    let len2 = ph2.len();

    let ph = ph2.merge(ph1);
    assert_eq!(len1 + len2, ph.len());
    let min_prio = ph.find_min_with_prio();
    assert!(min_prio.is_some());
    let (k, p) = min_prio.unwrap();
    assert_eq!(1, k);
    assert_eq!(1, p);
}

#[test]
fn delete_min() {
    let mut ph = create_heap(1, 11);
    let mut len = ph.len();
    let mut tracker = 1;

    while len != 0 {
        assert_eq!(len, ph.len());

        let min_prio = ph.find_min_with_prio();
        assert!(min_prio.is_some());
        let (_, p) = min_prio.unwrap();
        assert_eq!(tracker, p);
        tracker += 1;

        let del_prio = ph.delete_min();
        assert_eq!(min_prio, del_prio);

        len = ph.len();
    }

    assert!(ph.find_min().is_none());
    assert_eq!(0, ph.len());
}
