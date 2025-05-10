class Solution:
  def lengthOfLastWord(self, s: str) -> int:
    words = s.split(" ")
    i = len(words)
    l = 0
    for w in range(i):
        if len(words[i - 1]) > 0:
            l = len(words[i - 1])
            break
        i -= 1
    return l
        
