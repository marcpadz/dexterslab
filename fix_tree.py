import re

with open("porting-specs/frontend_design.md", "r") as f:
    content = f.read()

# Fix the broken tree
broken_tree = """│   ├── memories/+page.svelte (Knowledge Graph)
│   ├── work/
│   │   ├── email/+page.svelte"""

fixed_tree = """│   ├── memories/+page.svelte (Knowledge Graph)
│   ├── notes/
│   │   ├── recent/+page.svelte
│   │   ├── all/+page.svelte
│   │   └── trash/+page.svelte
│   ├── work/
│   │   ├── email/+page.svelte"""

content = content.replace(broken_tree, fixed_tree)

with open("porting-specs/frontend_design.md", "w") as f:
    f.write(content)
print("Fixed tree")
