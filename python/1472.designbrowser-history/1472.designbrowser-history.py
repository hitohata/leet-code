class BrowserHistory:

    def __init__(self, homepage: str):
        self.history = History(homepage)

    def visit(self, url: str) -> None:
        self.history.after = History(url, self.history)
        self.history = self.history.after

    def back(self, steps: int) -> str:
        while self.history.before and steps > 0:
            self.history = self.history.before
            steps -= 1
        return self.history.current

    def forward(self, steps: int) -> str:
        while self.history.after and steps > 0:
            self.history = self.history.after
            steps -= 1
        return self.history.current

class History:
    def __init__(self, current: str, before = None, after = None):
        self.current = current
        self.before = before
        self.after = after


# Your BrowserHistory object will be instantiated and called as such:
# obj = BrowserHistory(homepage)
# obj.visit(url)
# param_2 = obj.back(steps)
# param_3 = obj.forward(steps)
