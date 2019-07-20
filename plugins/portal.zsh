function pp() { 
    # p is used for 'print' in zsh
    result=`portal jump $1`
    if [[ "$result" =~ "Not found" ]]; then
      echo $result
    else 
      cd $result
    fi
}

# Hook cd command to run tracker every change of dir
autoload -U add-zsh-hook
add-zsh-hook -Uz chpwd (){ 
    portal db track $pwd
}
