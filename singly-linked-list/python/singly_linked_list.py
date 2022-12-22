class Node:
    def __init__(self, value: int, next_node):
        self.value = value
        self.next_node = next_node
        
class SinglyLinkedList:
    def __init__(self):
        self.head = None
        self.length = 0

    def get(self, index: int) -> int:
        if (index > self.length - 1):
            return -1
        
        next_node = self.head
        
        for i in range(index):
            next_node = next_node.next_node
            
        return next_node.value

    def addAtHead(self, val: int) -> None:
        if (self.length == 0):
            self.head = Node(val, None)
        else:
            self.head = Node(val, self.head)
            
        self.length += 1

    def addAtTail(self, val: int) -> None:
        if (self.length == 0):
            self.head = Node(val, None)
        else:
            next_node = self.head
            
            while (next_node.next_node != None):
                next_node = next_node.next_node
                
            next_node.next_node = Node(val, None)
            
        self.length += 1
            

    def addAtIndex(self, index: int, val: int) -> None:
        if (index > self.length):
            return
        
        if (index == 0):
            self.head = Node(val, self.head)
        else:
            next_node = self.head

            for i in range(index - 1):
                next_node = next_node.next_node

            node_being_added = Node(val, next_node.next_node)
            next_node.next_node = node_being_added
        
        self.length += 1

    def deleteAtIndex(self, index: int) -> None:
        if (index >= self.length):
            return
        
        if (index == 0):
            self.head = self.head.next_node
        else:
            next_node = self.head

            for i in range(index - 1):
                print(next_node.value)
                next_node = next_node.next_node

            pending_next_node = next_node.next_node.next_node
            next_node.next_node = pending_next_node
        
        self.length -= 1
