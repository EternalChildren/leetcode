// A binary tree is univalued if every node in the tree has the same value.

// Return true if and only if the given tree is univalued.


public class TreeNode {
    public var val: Int
    public var left: TreeNode?
    public var right: TreeNode?
    public init(_ val: Int) {
        self.val = val
        self.left = nil
        self.right = nil
    }
}
let node3 = TreeNode(5)
let node4 = TreeNode(2)
let node1 = TreeNode(2)
let node2 = TreeNode(2)
let node0 = TreeNode(2)

node0.left = node1
node0.right = node2
node1.left = node3
node1.right = node4

extension TreeNode {
    // MARK: -中序遍历
    func traverseInOrder(process: (Int) -> Void) {
        left?.traverseInOrder(process: process)
        process(val)
        right?.traverseInOrder(process: process)
    }
    // MARK: -前序遍历
    func traversePreOrder(process: (Int) -> Void) {
        process(val)
        left?.traverseInOrder(process: process)
        right?.traverseInOrder(process: process)
    }
    // MARK: -后序遍历
    func traversePostOrder(process: (Int) -> Void) {
        left?.traverseInOrder(process: process)
        right?.traverseInOrder(process: process)
        process(val)
    }
}

// 12ms
func isUnivalTree(_ root: TreeNode?) -> Bool {
    guard root != nil else { return false }
    let n = root!.val
    var status = true 
    root!.traverseInOrder(){ i in
        if i != n {
            status = false
        }
    }
    return status
}

isUnivalTree(node0)
