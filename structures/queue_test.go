package structures

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestValidatorShouldSucceed(t *testing.T) {
	for scenario, fn := range map[string]func(t *testing.T){
		"successfully enqueued an int": enqueueIntIsSuccessful,
		"successfully dequeued an int": dequeueIntIsSuccessful,
		"successfully peek":            peekIntIsSuccessful,
	} {
		t.Run(scenario, func(t *testing.T) {
			fn(t)
		})
	}
}

func enqueueIntIsSuccessful(t *testing.T) {
	q := newQueue()
	q.enqueue(32)
	assert.Equal(t, 32, q.peek())
}

func dequeueIntIsSuccessful(t *testing.T) {
	q := newQueue()
	q.enqueue(12)
	q.enqueue(32)
	q.enqueue(22)
	q.enqueue(42)
	q.enqueue(52)
	assert.Equal(t, 12, q.dequeue())
	assert.Equal(t, 32, q.peek())
}

func peekIntIsSuccessful(t *testing.T) {
	q := newQueue()
	q.enqueue(42)
	assert.Equal(t, 42, q.peek())

	q.dequeue()
	assert.Equal(t, 0, q.peek())
}
