# Get project numbers and read them line by line
gh project list --owner somossoftrek --format json -q .projects[].number | while read -r project_number; do
    echo "Processing project number: $project_number"
    cargo run $project_number
    # Add your processing logic here for each project number
done