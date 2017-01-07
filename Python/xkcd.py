import random
import argparse
import sys

def get_args(args=None):
    parser = argparse.ArgumentParser(description="Generate a password XKCD style")
    parser.add_argument('-c', '--count', help="Number of words", default=6, type=int)
    parser.add_argument('-w', '--wordlist', help="Specific wordlist to use", default="../wordlist.txt")

    result = parser.parse_args(args)
    return (result.count, result.wordlist)

def get_random_words(wordlist, count):
    with open(wordlist,'r') as f:
        content = f.read()
    content = content.split()
    words = []

    for i in range(count):
        word = content[random.randint(0, len(content))]
        words.append(word)

    print(' '.join(words))

if __name__ == '__main__':
    c, w = get_args(sys.argv[1:])
    get_random_words(w, c)


