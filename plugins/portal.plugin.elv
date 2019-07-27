PORTAL_DEBUG=0
PORTAL_DIR=/usr/local/lib/portal
PORTAL_LOG=$PORTAL_DIR/portal.log

fn p [dir]{
  result=(portal jump $dir)
  if (has-prefix "$result" "Error") {
    put $result
  } else  {
    cd $result
  }
}

fn track []{
  if (== $PORTAL_DEBUG 1) {
    echo "Debugging tracker"
    portal db track $pwd
  } else {
    mkdir -p $PORTAL_DIR
    touch $PORTAL_LOG
    portal db track $pwd >> $PORTAL_LOG
    echo "--------------------------------------------" >> $PORTAL_LOG
  }
}

after-chdir = [[_]{ 
  track
}]
