source env.fish
if test -d $target_dir
    echo "Directory $target_dir already exists.\n";
    read -lP "Want to wipe and install? (y/N)" confirm
    if test $confirm = "y"
        rm -rf "$target_dir/*"
    else
        echo "exiting"
        return
    end
end

mkdir -p "$target_dir/template"
cp -r template/* "$target_dir/template"
cp "$bin_name" "$target_dir"
if !test -d $link_dir
    mkdir -p $link_dir
end
ln -sf "$target_dir/$bin_name" "$link_loc"
set -U fish_user_paths "/usr/local/bin"
echo "/usr/local/bin has been added to path, just so you know"
