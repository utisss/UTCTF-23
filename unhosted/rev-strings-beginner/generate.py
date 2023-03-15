'''
Script to generate the problem.
'''

c_source_code: str = "#include <stdio.h>\n"

def process_file(filepath: str) -> str:
    with open(filepath, "r") as infile:
        text = infile.read()
    return text.replace("\n", "\\n").replace('"', '”')

# Make some of the strings global
c_source_code += f"char *taleOfTwoCities = \"{process_file('dickens.txt')}\";\n"
c_source_code += f"char *beeMovie = \"{process_file('beemovie.txt')}\";\n"
c_source_code += f"char *pi = \"{process_file('pi.txt')}\";\n"
# Hide the real flag in the data segment
c_source_code += f"char *flag = \"{process_file('flag.txt')}\";\n"

c_source_code += "int main(int argc, char** argv) {\n"
# Make some of the strings local
c_source_code += f"    char *darthPlagueis = \"{process_file('sith.txt')}\";\n"
c_source_code += f"    char *sydney = \"{process_file('bing.txt')}\";\n"
c_source_code += f"    char *fitnessGram = \"{process_file('fitness.txt')}\";\n"
c_source_code += f"    char *fakeFlag = \"{process_file('fakeFlag.txt')}\";\n"
# Now for code that actually does stuff when the program is run
with open("template.txt", "r") as infile:
    c_source_code += infile.read()
c_source_code += "\n}"

with open("readingList.c", "w") as outfile:
    outfile.write(c_source_code)