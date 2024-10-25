#given the last score, filters the wordlist
def filter_wordlist(score, word_list):
    word = score["word"]
    marks = score["marks"]
    # keep track of which letters are contained in, absent from, and in the correct position in the target word
    contained, correct, absent = [], {}, []
    for i in range(len(word)):
        l = word[i]
        m = marks[i]
        if m==2:
            correct[i] = l
        elif m==1:
            contained.append(l)
        else:
            absent.append(l)
    # filter out absent for letters in contained and absent
    absent = [a for a in absent if (a not in contained and a not in correct.values())]
    def good_word(w):
        if w==word:
            return False
        for a in absent:
            if a in w:
                return False
        for c in contained:
            if c not in w:
                return False
        for i,l in correct.items():
            if w[i] != l:
                return False
        return True
    new_list = [w for w in word_list if good_word(w)]
    return new_list