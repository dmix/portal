fn p [dir]{
  result=(portal jump $dir)
  if (has-prefix "$result" "Error") {
    put $result
  } else  {
    cd $result
  }
}

after-chdir = [[_]{ 
  portal db track $pwd
}]
