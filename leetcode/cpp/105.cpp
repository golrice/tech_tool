using namespace std;
#include <vector>
#include <queue>
#include <unordered_map>
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
/*
给定两个整数数组 preorder 和 inorder
其中 preorder 是二叉树的先序遍历
inorder 是同一棵树的中序遍历
请构造二叉树并返回其根节点。
*/
struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution
{
    std::unordered_map<int, int> index;

public:
    TreeNode *buildTree(vector<int> &preorder, vector<int> &inorder)
    {
        int n = preorder.size();
        for (int i = 0; i < n; ++i)
        {
            index[inorder[i]] = i;
        }
        return help(preorder, inorder, 0, n - 1, 0, n - 1);
    }
    TreeNode *help(const vector<int> &pre, const vector<int> &in, int pl, int pr, int il, int ir)
    {
        if (pl > pr)
            return nullptr;

        // 找到根节点位置
        int root_index = pl;
        // 创建根节点
        TreeNode *root = new TreeNode(pre[pl]);
        // 找到中序遍历中根节点的位置
        int in_root_index = index[pre[pl]];
        // 确定左子树的大小
        const int left_size = in_root_index - pl;
        // 递归地找到完成左右子树
        root->left = help(pre, in, root_index + 1, root_index + left_size, il, in_root_index - 1);
        root->right = help(pre, in, pl + left_size + 1, pr, in_root_index + 1, ir);

        return root;
    }
};