import pytest
import heapq


if True:

    def max_sliding_window(nums: list[int], k:int) -> list[int]:
        if len(nums) < k:
            return []
        local_maximums:list[int] = []

        window_min_heap = [(-val, ix) for ix,val in enumerate(nums[:k-1])]
        heapq.heapify(window_min_heap)

        for i in range(len(nums)-k+1):
            print("processing", nums[i:i+k])
            # we can process nums[i:i+k]
            new_item = nums[i+k-1]
            print("  adding elt ", new_item)
            heapq.heappush(window_min_heap, (-new_item, i+k-1))
            print("  heap ", window_min_heap)
            while True:
                # ersatz of do-while loop
                min_val, min_ix = window_min_heap[0]
                if min_ix < i:
                    # outside of current range
                    heapq.heappop(window_min_heap)
                else:
                    break
            assert i <= min_ix < i+k
            print("found max ", -min_val, "@ index", min_ix)
            local_maximums.append(-min_val)

        return local_maximums





@pytest.mark.parametrize(("numbers", "k", "expected"), [
    ([1,2,3], 1, [1,2,3]),
    ([2,4,6,1,3, 2], 3, [6, 6, 6, 3]),
    ([-k for k in range(20)], 10, [-k for k in range(11)]),
    ])
def test_max_sliding_window(numbers, k, expected):
    assert max_sliding_window(numbers, k) == expected
