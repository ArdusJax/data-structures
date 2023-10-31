package structures

// Node - Node is a data type that is contained by the Queue
type Node struct {
	data int
	next *Node
}

func newNode(data int) *Node {
	return &Node{
		data: data,
		next: nil,
	}
}

// Queue - Queue is a common data structure that is used in programming based on a linked list
type Queue struct {
	head   *Node
	tail   *Node
	length int
}

func newQueue() Queue {
	return Queue{
		head:   nil,
		tail:   nil,
		length: 0,
	}
}

func (q *Queue) enqueue(data int) {
	node := newNode(data)
	if q.tail != nil {
		q.tail.next = node
		q.tail = node
		q.length++
	} else {
		q.head = node
		q.tail = node
		q.length++
	}
}

func (q *Queue) dequeue() int {
	if q.head != nil {
		data := q.head.data
		q.head = q.head.next
		q.length--
		return data
	}

	return 0
}

func (q *Queue) peek() int {
	if q.head != nil {
		return q.head.data
	}

	return 0
}
