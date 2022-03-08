package example

import (
	"container/heap"
	"testing"
)

func TestHeap(t *testing.T) {
	h := &Heap{2, 1, 5}
	heap.Init(h)
	heap.Push(h, 3)
	t.Logf("minimum: %d\n", (*h)[0])
	t.Log((*h)[0], (*h)[1], (*h)[2], (*h)[3])
	for h.Len() > 0 {
		t.Logf("%d ", heap.Pop(h))
	}
}
