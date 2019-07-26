#!/usr/bin/env zsh

export PORTAL_DEBUG=false
export PORTAL_DIR=/usr/local/lib/portal

function z() { 
    # p is used for 'print' in zsh, so using z
    result=`portal jump $1`
    if [[ "$result" =~ "Not found" ]]; then
      echo $result
    else 
      cd $result
    fi
}

function track() {
    if [[ PORTAL_DEBUG == true ]] {
        portal db track $pwd
    } else {
	touch $PORTAL_DIR/portal.log
        portal db track $pwd >> $PORTAL_DIR/portal.log
	echo "-----------------------------------------------------------"
    }
}

# Hook cd command to run tracker every change of dir
autoload -U add-zsh-hook
add-zsh-hook -Uz chpwd (){ 
    track
}
