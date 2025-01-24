class MyStack:

    def __init__(self):
        self.queue = []
        

    def push(self, x: int) -> None:
        len_queue = len(self.queue)
        if len_queue > 0:
            self.queue.append(x)
            for i in range(len_queue):
                v = self.queue.pop(0)
                self.queue.append(v)
        else:
            self.queue.append(x)
        

    def pop(self) -> int:
        return self.queue.pop(0)

    def top(self) -> int:
        return self.queue[0]
        

    def empty(self) -> bool:
        if len(self.queue) > 0:
            return False
        else:
            return True
        


# Your MyStack object will be instantiated and called as such:
# obj = MyStack()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.top()
# param_4 = obj.empty()
