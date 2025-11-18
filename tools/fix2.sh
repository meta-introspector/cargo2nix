fix () {
    #F="$(cat $1)/config";
    #echo $F
    #echo "${F}/config";
    URL=$(grep $1 .gitmodules -A1 | grep url | cut -d= -f2)
    git submodule add $URL $1
}

fix  submodules/approx
fix  submodules/ar_archive_writer
fix  submodules/async-fs
fix  submodules/async-process
fix  submodules/async-signal
fix  submodules/automod
fix  submodules/build-alert
fix  submodules/color-print
fix  submodules/dbus-rs
fix  submodules/dissimilar
fix  submodules/dunce
fix  submodules/gg-alloc
fix  submodules/humantime
fix  submodules/indoc
fix  submodules/itertools-num
fix  submodules/libredox
fix  submodules/md5
fix  submodules/minicov
fix  submodules/no-panic
fix  submodules/num-complex
fix  submodules/serde-stacker
fix  submodules/sha1-smol
fix  submodules/smol
fix  submodules/stacker
fix  submodules/syscall
fix  submodules/unicode-bom
