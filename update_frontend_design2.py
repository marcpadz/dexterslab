import re

with open("porting-specs/frontend_design.md", "r") as f:
    content = f.read()

# Remove Models, Settings, and Subscription from text
content = content.replace("- **Models**: Local and cloud model selection.\n", "")
content = content.replace("Settings, Subscription.", "")

# Remove from Appendix A file tree
content = re.sub(r'\s*├── models/\+page\.svelte\n', '\n', content)
content = re.sub(r'\s*├── settings/\+page\.svelte\n', '\n', content)
content = re.sub(r'\s*└── subscription/\+page\.svelte\n', '\n', content)

with open("porting-specs/frontend_design.md", "w") as f:
    f.write(content)
print("Updated frontend_design.md")
