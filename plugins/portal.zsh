function pp() { 
    # p is used for 'print' in zsh
    result=`portal jump $1`
    if [[ "$result" =~ "Not found" ]]; then
      echo $result
    else 
      cd $result
    fi
}
