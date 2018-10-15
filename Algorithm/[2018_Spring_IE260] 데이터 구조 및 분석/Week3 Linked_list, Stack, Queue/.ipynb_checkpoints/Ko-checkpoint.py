class Node:
    node_next = ''
    obj_value = ''
    bln_head = False
    bln_tail = False
    
    def __init__(self,obj_value = '', node_next = '', bln_head = False, bln_tail = False):
        self.node_next = node_next
        self.obj_value = obj_value
        self.bln_head = bln_head
        self.bln_tail = bln_tail
    
    def get_value(self):
        return self.obj_value
    
    def set_value(self, obj_value):
        self.obj_value = obj_value
        return
    
    def get_next(self):
        return self.node_next
    
    def set_next(self, node_next):
        self.node_next = node_next
        return
        
    def is_head(self):
        return self.bln_head
    
    def is_tail(self):
        return self.bln_tail


class SinglyLinkedList:
    node_head = ''
    node_tail = ''
    
    size = 0
    
    def __init__(self):
        self.node_tail = Node(bln_tail = True)
        self.node_head = Node(bln_head= True, node_next= self.node_tail)
        return 
    
    def get(self, idx_retrieve):
        node_return = self.node_head
        
        for itr in range(idx_retrieve + 1):
            node_return = node_return.get_next()
            
        return node_return
    
    
    def insert_at(self, obj_insert, idx_insert):
        node_new = Node(obj_value= obj_insert)
        node_prev = self.get(idx_insert - 1)
        node_next = node_prev.get_next()
        
        node_prev.set_next(node_new)
        node_new.set_next(node_next)
        
        self.size += 1
        return
        
    def remove_at(self, idx_remove):
        node_prev = self.get(idx_remove - 1)
        node_remove = node_prev.get_next()
        node_next = node_remove.get_next()
        
        node_prev.set_next(node_next)
        
        self.size -= 1
        return node_remove.get_value()
    
    def print_status(self):
        node_current = self.node_head
        
        while node_current.get_next().is_tail() == False:
            node_current = node_current.get_next()
            print(node_current.get_value(), end = ' ')
        return
    
    
    def get_size(self):
        return self.size
        