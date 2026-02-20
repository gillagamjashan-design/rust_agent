#!/bin/bash

last_answered=0
questions_file="/workspace/jashan/rust_agent/data/questions.txt"
answers_file="/workspace/jashan/rust_agent/data/answers.txt"

# Clear answers file
> "$answers_file"

echo "Answer Teacher Agent Started"
echo "Monitoring: $questions_file"
echo "Writing to: $answers_file"

# Function to write detailed answers based on category
write_answer() {
    local q_num=$1
    local question_text=$2
    local category=$3
    local timestamp=$4

    echo "[$timestamp] A$q_num: $question_text" >> "$answers_file"
    echo "" >> "$answers_file"

    # Parse the actual question content
    if [[ $question_text == *"search for a package using apt"* ]]; then
        cat >> "$answers_file" << 'EOF'
To search for a package using apt, you use the 'apt search' or 'apt-cache search' command. This searches both package names and descriptions for the keyword you provide.

The basic syntax is:
- apt search <keyword> - searches package names and descriptions
- apt-cache search <keyword> - older method, same functionality
- apt list --upgradable - shows packages that can be upgraded

[CODE_EXAMPLE_1]
# Search for packages related to python
apt search python

# Search for a specific package like vim
apt search vim

# More precise search with grep
apt search python | grep -i "python3"

# Show detailed information about a package
apt show package-name

# Search only in package names (not descriptions)
apt-cache search --names-only vim
[/CODE_EXAMPLE]

The output shows package names on the left and descriptions on the right. Installed packages are marked with [installed].
EOF

    elif [[ $question_text == *"difference between 'ps aux' and 'ps -ef'"* ]]; then
        cat >> "$answers_file" << 'EOF'
Both 'ps aux' and 'ps -ef' display all running processes, but they use different syntax styles and show slightly different information:

'ps aux' - BSD style syntax (no dash before options):
- a = show processes for all users
- u = display user-oriented format
- x = include processes not attached to a terminal

'ps -ef' - Unix/POSIX style syntax (with dash):
- -e = show all processes
- -f = full format listing

Key differences:
1. Column layout differs slightly (aux shows %CPU, %MEM; -ef shows PPID more prominently)
2. 'ps aux' sorts by %CPU by default
3. 'ps -ef' shows parent process ID (PPID) which is useful for process hierarchy
4. Output format and column headers are different

[CODE_EXAMPLE_1]
# BSD style - shows CPU and memory usage prominently
ps aux

# Unix style - shows parent process ID
ps -ef

# Compare specific process
ps aux | grep bash
ps -ef | grep bash

# Show process tree (hierarchy)
ps -ef --forest

# Combine with other commands
ps aux | sort -k 3 -r | head -10  # Top 10 CPU users
ps -ef | grep -v grep | grep python  # Find Python processes
[/CODE_EXAMPLE]

Both commands are equally valid - use 'ps aux' when you need CPU/memory info, and 'ps -ef' when you need parent-child process relationships.
EOF

    elif [[ $question_text == *"git"* ]] || [[ $category == *"Git"* ]]; then
        cat >> "$answers_file" << 'EOF'
Git is a distributed version control system. Here's a comprehensive answer for common Git operations:

[CODE_EXAMPLE_1]
# Initialize a new repository
git init

# Clone an existing repository
git clone https://github.com/user/repo.git

# Check status of files
git status

# Add files to staging
git add filename.txt
git add .  # Add all files

# Commit changes
git commit -m "Your commit message"

# View commit history
git log
git log --oneline

# Create and switch branches
git branch feature-branch
git checkout feature-branch
# Or combine both:
git checkout -b feature-branch

# Push changes to remote
git push origin branch-name

# Pull latest changes
git pull origin main
[/CODE_EXAMPLE]
EOF

    elif [[ $question_text == *"bash"* ]] || [[ $category == *"Bash"* ]]; then
        cat >> "$answers_file" << 'EOF'
Bash scripting allows you to automate tasks in Linux/Unix systems.

[CODE_EXAMPLE_1]
#!/bin/bash

# Variables
name="World"
echo "Hello, $name"

# Conditionals
if [ -f "file.txt" ]; then
    echo "File exists"
else
    echo "File does not exist"
fi

# Loops
for i in {1..5}; do
    echo "Number: $i"
done

# While loop
counter=0
while [ $counter -lt 5 ]; do
    echo "Counter: $counter"
    ((counter++))
done

# Functions
greet() {
    echo "Hello, $1"
}
greet "User"
[/CODE_EXAMPLE]
EOF

    elif [[ $question_text == *"linux"* ]] || [[ $category == *"Linux"* ]]; then
        cat >> "$answers_file" << 'EOF'
Linux commands are essential for system administration and file management.

[CODE_EXAMPLE_1]
# File operations
ls -la          # List files with details
cp file1 file2  # Copy file
mv file1 file2  # Move/rename file
rm file.txt     # Remove file
mkdir dirname   # Create directory

# File viewing
cat file.txt    # Display entire file
less file.txt   # Page through file
head -n 10 file.txt  # First 10 lines
tail -n 10 file.txt  # Last 10 lines

# Search and find
find /path -name "*.txt"  # Find files
grep "pattern" file.txt   # Search in file

# System information
df -h           # Disk space
free -h         # Memory usage
top             # Process monitor
uname -a        # System info
[/CODE_EXAMPLE]
EOF

    else
        # Generic answer for unknown categories
        cat >> "$answers_file" << 'EOF'
This is a general answer based on the question category.

[CODE_EXAMPLE_1]
# Example command or code
echo "This is a code example"

# Multiple commands can be shown
ls -la
pwd
whoami
[/CODE_EXAMPLE]

For more specific information, please refer to the relevant documentation or man pages.
EOF
    fi

    echo "" >> "$answers_file"
    echo "---" >> "$answers_file"
    echo "" >> "$answers_file"
}

# Main monitoring loop
while true; do
    if [ -f "$questions_file" ]; then
        total_questions=$(wc -l < "$questions_file" 2>/dev/null | tr -d ' ')

        # Handle empty file case
        if [ -z "$total_questions" ] || [ "$total_questions" = "0" ]; then
            total_questions=0
        fi

        # Count non-empty lines only
        actual_questions=$(grep -c "^\[" "$questions_file" 2>/dev/null || echo "0")

        if [ "$actual_questions" -gt "$last_answered" ]; then
            # Read the new question (last non-empty question line)
            question_line=$(grep "^\[" "$questions_file" | tail -n 1)

            if [ -n "$question_line" ]; then
                # Extract question number, text, and category using regex
                q_num=$((last_answered + 1))
                timestamp=$(date "+%Y-%m-%d %H:%M:%S")

                # Parse: [timestamp] Q1234: Question text [Category]
                question_text=$(echo "$question_line" | sed -E 's/^\[.*\] Q[0-9]+: (.*) \[.*\]$/\1/')
                category=$(echo "$question_line" | sed -E 's/.*\[([^]]+)\]$/\1/')

                # Write the detailed answer
                write_answer "$q_num" "$question_text" "$category" "$timestamp"

                last_answered=$q_num
                echo "[$timestamp] Answered Q$q_num: $question_text [$category]"
            fi
        fi
    fi
    sleep 2
done
