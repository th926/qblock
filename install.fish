source env.fish
if test -d $target_dir
    echo "Directory $target_dir already exists.\n";
    while true
        read "-l -p Want to wipe and install? (y/N)" confirm
        if [$confirm = "y"]
            rm -rf "$target_dir/*"
        else
            echo "exiting"
            return
        end
    end
end

mkdir -p "$target_dir/template"
cp "template/*" "$target_dir/template"
cp "$bin_name" "$target_dir"
ln "-sf $target_dir/$bin_name" "$link_loc"
