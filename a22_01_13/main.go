package main

import "fmt"

func main() {
	nums := []int{0,1,2,2,3,0,4,2}
	val := 2
	k := removeElement(nums, val)
	println(fmt.Sprintf("%d, %v", k, nums))
}


func removeElement(nums []int, val int) int {
	originalLen := len(nums)
	shiftPositions := 0
	for i := range (nums) {
		if nums[i] == val {
			shiftPositions++
			continue
		}
		nums[i - shiftPositions] = nums[i]
	}
	return originalLen - shiftPositions
}