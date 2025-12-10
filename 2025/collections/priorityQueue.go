package collections

import (
	"errors"
	"fmt"
)

// An Node is the backbone for the priority queue.
type Node[T any, P int | float32 | float64] struct {
	value    T // The value of the item; arbitrary.
	priority P // The priority of the item in the queue.
	next     *Node[T, P]
	prev     *Node[T, P]
}

func (n1 *Node[T, P]) swap(n2 *Node[T, P]) {
	n1.priority, n2.priority = n2.priority, n1.priority
	n1.value, n2.value = n2.value, n1.value
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue[T any, P int | float32 | float64] struct {
	head   *Node[T, P]
	tail   *Node[T, P]
	length int
}

func (pq PriorityQueue[T, P]) GetHead() *Node[T, P] {
	return pq.head
}

func (pq PriorityQueue[T, P]) GetTail() *Node[T, P] {
	return pq.tail
}

func (pq PriorityQueue[T, P]) Len() int { return pq.length }

func (pq *PriorityQueue[T, P]) PushRaw(x T, priority P) {
	pq.head = &Node[T, P]{value: x, priority: priority, next: pq.head, prev: nil}

	if pq.length == 0 {
		pq.tail = pq.head
	}
	if pq.head.next != nil {
		pq.head.next.prev = pq.head
	}

	pq.length++
}

func (pq *PriorityQueue[T, P]) partition(low, high *Node[T, P]) *Node[T, P] {
	pivot := high.priority
	i := low.prev

	curr := low

	for curr != high {
		if curr.priority <= pivot {
			if i == nil {
				i = low
			} else {
				i = i.next
			}

			i.swap(curr)
		}

		curr = curr.next
	}

	if i == nil {
		i = low
	} else {
		i = i.next
	}

	i.swap(high)

	return i
}

func (pq *PriorityQueue[T, P]) quickSort(low, high *Node[T, P]) {
	//Base case: stop when invalid range
	if low != nil && high != nil && low != high && low != high.next {
		// Partition the list and get the pivot node
		pivot := pq.partition(low, high)

		// Recursively sort the left half
		pq.quickSort(low, pivot.prev)

		// Recursively sort the right half
		pq.quickSort(pivot.next, high)
	}

}

func (pq *PriorityQueue[T, P]) Sort() {
	pq.quickSort(pq.head, pq.tail)
}

func (pq *PriorityQueue[T, P]) Push(x T, priority P) {
	var curr *Node[T, P]

	for curr = pq.head; curr != nil && curr.next != nil && curr.priority < priority; curr = curr.next {
	}

	if pq.length == 0 {
		// New head.
		pq.head = &Node[T, P]{value: x, priority: priority, next: nil, prev: nil}
	} else if curr.next == nil && curr.priority < priority {
		// New tail.
		curr.next = &Node[T, P]{value: x, priority: priority, next: nil, prev: curr}
	} else {
		new := &Node[T, P]{value: x, priority: priority, next: curr, prev: curr.prev}

		if curr.prev != nil {
			curr.prev.next = new
		} else {
			pq.head = new
		}
		curr.prev = new
	}

	pq.length++
}

func (pq *PriorityQueue[T, P]) Pop() (T, error) {
	head := pq.head

	if head == nil {
		return *new(T), errors.New("this PriorityQueue is empty")
	}

	pq.head = pq.head.next

	if pq.head != nil {
		pq.head.prev = nil
	}

	pq.length--

	return head.value, nil
}

func (pq *PriorityQueue[T, P]) Dump() {
	for curr := pq.head; curr != nil; curr = curr.next {
		fmt.Printf("Item: %v, Priority: %v\n", curr.value, curr.priority)
	}
}

// TODO: Decide if this needed
// update modifies the priority and value of an Item in the queue.
// func (pq *PriorityQueue[T]) Update(item *Item[T], value T, priority float64) {
// 	item.value = value
// 	item.priority = priority
// 	heap.Fix(pq, item.index)
// }
