from enum import Enum
import functools
import json
import pprint

class ORDER(Enum):
    CORRECT=1,
    SAME=0,
    INCORRECT=-1,

def cmp(a: list, b: list) -> ORDER:
    for i in range(0, min(len(a),len(b))):
        left = a[i]
        right = b[i]

        if type(left) == int and type(right) == int:
            if left < right:
                return ORDER.CORRECT
            elif left > right:
                return ORDER.INCORRECT
        
        elif type(left) == int and type(right) == list:
            left = [a[i]]

        elif type(left) == list and type(right) == int:
            right = [b[i]]


        if type(left) == list and type(right) == list:
            res = cmp(left,right)
            if res != ORDER.SAME:
                return res


    if len(a) < len(b):
        return ORDER.CORRECT
    elif len(a) > len(b):
        return ORDER.INCORRECT
    
    return ORDER.SAME

def cmp_int(a: list, b: list) -> int:
    return cmp(a,b).value[0]


def main():
    test()
    correct_sum = 0
    packets = [
        [[2]],
        [[6]]
    ]
    with open("src/input","r") as f:
        pair = 1
        while True:
            a_str = f.readline()
            if not a_str or a_str == '':
                break
            a = json.loads(a_str)
            b = json.loads(f.readline())
            if cmp(a,b) == ORDER.CORRECT:
                correct_sum += pair
            f.readline()
            packets.append(a)
            packets.append(b)

            pair += 1
        f.close()
    print(f"PART 1 {correct_sum}")

    packets.sort(key=functools.cmp_to_key(cmp_int), reverse=True)
    # pprint.pp(packets)
    print(f"PART 2 {(packets.index([[2]]) + 1) * (packets.index([[6]]) + 1)}")

def test():
    assert ORDER.CORRECT == cmp([1,1,3,1,1],[1,1,5,1,1])
    assert ORDER.CORRECT == cmp([[1],[2,3,4]],[[1],4])
    assert ORDER.INCORRECT == cmp([9],[[8,7,6]])
    assert ORDER.CORRECT == cmp([[4,4],4,4],[[4,4],4,4,4])
    assert ORDER.INCORRECT == cmp([7,7,7,7],[7,7,7])
    assert ORDER.CORRECT == cmp([],[3])
    assert ORDER.INCORRECT == cmp([[[]]],[[]])
    assert ORDER.INCORRECT == cmp([1,[2,[3,[4,[5,6,7]]]],8,9],[1,[2,[3,[4,[5,6,0]]]],8,9])

if __name__=="__main__":
    main()