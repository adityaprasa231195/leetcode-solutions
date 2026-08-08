class Solution {
public:
    vector<int> shuffle(vector<int>& nums, int n) {
        vector<int> target;

        for (int i = 0; i < n; i++) {
            target.push_back(nums[i]);
            target.push_back(nums[i + n]);
        }

        return target;
    }
};