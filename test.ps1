cat .\xmas_tree.txt | sub -f '#F4F556' i i | sub -f "#009302" -rc '([\.bd$])' '&1' | sub -f '#FFC300' -rc '([v<>XA])' '&1' | sub -f "#733719" '#' '#' | sub -f "FF5533" '@' '@' | sub -f "#0F73D9" '\' '\'