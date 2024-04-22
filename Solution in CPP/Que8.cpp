// Given a binary tree, implement a function that returns the maximum depth of the tree
 

// TreeNode represents the node of a tree.

int maxDepth(TreeNode *root){
    if(root == NULL){
        return 0;
    }

    int HeightLeft = maxDepth(root->left);
    int HeightRight = maxDepth(root->right);
    return max(HeightLeft, HeightRight)+1;
}