
from typing import Callable


class Directory:
    def __init__(self: "Directory", parent: "Directory|None", name: str) -> None:
        self.name=name
        self.parent=parent
        self.subdirectories: dict[str,Directory] = dict()
        self.files: dict[str,int] = dict()
        self.size=0
    
    def update_size(self: "Directory") -> int:
        for dir in self.subdirectories.values():
            self.size += dir.update_size()
        
        self.size += sum(self.files.values())

        return self.size

    def match_expr(self: "Directory", expr:Callable[["Directory"],bool]) -> list:
        matched: list[Directory] = []
        if expr(self):
            matched.append(self)
        
        for dir in self.subdirectories.values():
            matched.extend(dir.match_expr(expr))

        return matched

    def __lt__(self: "Directory", other: "Directory")->bool:
         return self.size < other.size
    
    def __str__(self) -> str:
        return self.to_str("  ")

    def to_str(self:"Directory", padding: str)->str:
        newline='\n'
        next_depth = "  " + padding
        return f"""{padding}{self.name} ({self.size})
{newline.join({f"{next_depth}{k} - {v}" for k,v in self.files.items()})}
{newline.join([f"{x.to_str(next_depth)}" for x in self.subdirectories.values()])}
"""

P1_MAX_SIZE = 100000
P2_DISK_SIZE = 70000000
P2_TARGET_FREE_SIZE = 30000000

def main():
    directory = treeify()
    directory.update_size()
    # print(directory)
    print(f"PART 1 {sum([x.size for x in directory.match_expr(lambda d: d.size<=P1_MAX_SIZE)])}")
    
    free_size = P2_DISK_SIZE - directory.size
    required_size = P2_TARGET_FREE_SIZE - free_size
    deletion_candidates=directory.match_expr(lambda d: d.size>=required_size)
    deletion_candidates.sort()
    # print([f"{x.name} {x.size}" for x in deletion_candidates])
    print(f"PART 2 {deletion_candidates[0].size}")
    

def treeify() -> Directory:
    directory:Directory = Directory(None,"root")
    pwd:Directory=directory
    with open("src/input","r") as f:
        for line in f:
            tokens = line.strip().split(' ')
            if tokens[0] == '$':
                if tokens[1] == "cd":
                    if tokens[2] == "..":
                        pwd = pwd.parent
                    elif tokens[2] not in pwd.subdirectories:
                        new_dir = Directory(pwd, tokens[2])
                        pwd.subdirectories.update({tokens[2]: new_dir})
                        pwd = new_dir
                    else:
                        pwd = pwd.subdirectories.get(tokens[2])
                elif tokens[1] == "ls":
                    pass
            elif tokens[0].isdecimal():
                pwd.files.update({tokens[1]: int(tokens[0])})
    return directory

if __name__=="__main__":
    main()