import os
new_name = input("What do you want the new name of main.rs to be?\n")
if ".rs" not in new_name[-3:]:
    new_name+=".rs"
os.rename('main.rs', new_name)
with open("main.rs", "w") as f:
    pass