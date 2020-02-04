#include <vector>

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

using namespace std;
class Solution {
public:
    static ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int step = 0;
        auto pNode = new ListNode(0);
        ListNode* node = pNode;
        for (;l1 != nullptr || l2 != nullptr;) {
            int l1v = 0;
            int l2v = 0;
            if (l1) {
                l1v = l1->val;
                l1 = l1->next;
            }

            if (l2)
            {
                l2v = l2->val;
                l2 = l2->next;
            }

            int res = l1v + l2v + step;
            step = res / 10;
            res %= 10;
            node->val = res;

            if (l1 != nullptr || l2 != nullptr || step) {
                node->next = new ListNode(0);
                node = node->next;
            }

        }
        if (step) {
            node->val = step;
        }

        return pNode;
    }
};

int main() {
    ListNode l1 = ListNode(0);
    ListNode l2 = ListNode(0);
    l1.val = 6;
    l2.val = 7;
    ListNode* l3 = Solution::addTwoNumbers(&l1, &l2);
    return 0;
}