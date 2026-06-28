_dkp() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="dkp"
                ;;
            dkp,build)
                cmd="dkp__subcmd__build"
                ;;
            dkp,chunk)
                cmd="dkp__subcmd__chunk"
                ;;
            dkp,cross-refs)
                cmd="dkp__subcmd__cross__subcmd__refs"
                ;;
            dkp,diff)
                cmd="dkp__subcmd__diff"
                ;;
            dkp,eval)
                cmd="dkp__subcmd__eval"
                ;;
            dkp,export)
                cmd="dkp__subcmd__export"
                ;;
            dkp,fix)
                cmd="dkp__subcmd__fix"
                ;;
            dkp,generate)
                cmd="dkp__subcmd__generate"
                ;;
            dkp,get)
                cmd="dkp__subcmd__get"
                ;;
            dkp,graph)
                cmd="dkp__subcmd__graph"
                ;;
            dkp,help)
                cmd="dkp__subcmd__help"
                ;;
            dkp,info)
                cmd="dkp__subcmd__info"
                ;;
            dkp,init)
                cmd="dkp__subcmd__init"
                ;;
            dkp,inject)
                cmd="dkp__subcmd__inject"
                ;;
            dkp,install)
                cmd="dkp__subcmd__install"
                ;;
            dkp,keygen)
                cmd="dkp__subcmd__keygen"
                ;;
            dkp,l10n)
                cmd="dkp__subcmd__l10n"
                ;;
            dkp,list)
                cmd="dkp__subcmd__list"
                ;;
            dkp,mcp-manifest)
                cmd="dkp__subcmd__mcp__subcmd__manifest"
                ;;
            dkp,new)
                cmd="dkp__subcmd__new"
                ;;
            dkp,okf)
                cmd="dkp__subcmd__okf"
                ;;
            dkp,procedures)
                cmd="dkp__subcmd__procedures"
                ;;
            dkp,prompt)
                cmd="dkp__subcmd__prompt"
                ;;
            dkp,publish)
                cmd="dkp__subcmd__publish"
                ;;
            dkp,registry)
                cmd="dkp__subcmd__registry"
                ;;
            dkp,release-check)
                cmd="dkp__subcmd__release__subcmd__check"
                ;;
            dkp,review)
                cmd="dkp__subcmd__review"
                ;;
            dkp,rights)
                cmd="dkp__subcmd__rights"
                ;;
            dkp,run)
                cmd="dkp__subcmd__run"
                ;;
            dkp,search)
                cmd="dkp__subcmd__search"
                ;;
            dkp,serve)
                cmd="dkp__subcmd__serve"
                ;;
            dkp,sign)
                cmd="dkp__subcmd__sign"
                ;;
            dkp,skills)
                cmd="dkp__subcmd__skills"
                ;;
            dkp,tui)
                cmd="dkp__subcmd__tui"
                ;;
            dkp,uninstall)
                cmd="dkp__subcmd__uninstall"
                ;;
            dkp,update)
                cmd="dkp__subcmd__update"
                ;;
            dkp,validate)
                cmd="dkp__subcmd__validate"
                ;;
            dkp,webui)
                cmd="dkp__subcmd__webui"
                ;;
            dkp,yank)
                cmd="dkp__subcmd__yank"
                ;;
            dkp__subcmd__cross__subcmd__refs,help)
                cmd="dkp__subcmd__cross__subcmd__refs__subcmd__help"
                ;;
            dkp__subcmd__cross__subcmd__refs,list)
                cmd="dkp__subcmd__cross__subcmd__refs__subcmd__list"
                ;;
            dkp__subcmd__cross__subcmd__refs,validate)
                cmd="dkp__subcmd__cross__subcmd__refs__subcmd__validate"
                ;;
            dkp__subcmd__cross__subcmd__refs__subcmd__help,help)
                cmd="dkp__subcmd__cross__subcmd__refs__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__cross__subcmd__refs__subcmd__help,list)
                cmd="dkp__subcmd__cross__subcmd__refs__subcmd__help__subcmd__list"
                ;;
            dkp__subcmd__cross__subcmd__refs__subcmd__help,validate)
                cmd="dkp__subcmd__cross__subcmd__refs__subcmd__help__subcmd__validate"
                ;;
            dkp__subcmd__graph,help)
                cmd="dkp__subcmd__graph__subcmd__help"
                ;;
            dkp__subcmd__graph,list)
                cmd="dkp__subcmd__graph__subcmd__list"
                ;;
            dkp__subcmd__graph,stats)
                cmd="dkp__subcmd__graph__subcmd__stats"
                ;;
            dkp__subcmd__graph,validate)
                cmd="dkp__subcmd__graph__subcmd__validate"
                ;;
            dkp__subcmd__graph__subcmd__help,help)
                cmd="dkp__subcmd__graph__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__graph__subcmd__help,list)
                cmd="dkp__subcmd__graph__subcmd__help__subcmd__list"
                ;;
            dkp__subcmd__graph__subcmd__help,stats)
                cmd="dkp__subcmd__graph__subcmd__help__subcmd__stats"
                ;;
            dkp__subcmd__graph__subcmd__help,validate)
                cmd="dkp__subcmd__graph__subcmd__help__subcmd__validate"
                ;;
            dkp__subcmd__help,build)
                cmd="dkp__subcmd__help__subcmd__build"
                ;;
            dkp__subcmd__help,chunk)
                cmd="dkp__subcmd__help__subcmd__chunk"
                ;;
            dkp__subcmd__help,cross-refs)
                cmd="dkp__subcmd__help__subcmd__cross__subcmd__refs"
                ;;
            dkp__subcmd__help,diff)
                cmd="dkp__subcmd__help__subcmd__diff"
                ;;
            dkp__subcmd__help,eval)
                cmd="dkp__subcmd__help__subcmd__eval"
                ;;
            dkp__subcmd__help,export)
                cmd="dkp__subcmd__help__subcmd__export"
                ;;
            dkp__subcmd__help,fix)
                cmd="dkp__subcmd__help__subcmd__fix"
                ;;
            dkp__subcmd__help,generate)
                cmd="dkp__subcmd__help__subcmd__generate"
                ;;
            dkp__subcmd__help,get)
                cmd="dkp__subcmd__help__subcmd__get"
                ;;
            dkp__subcmd__help,graph)
                cmd="dkp__subcmd__help__subcmd__graph"
                ;;
            dkp__subcmd__help,help)
                cmd="dkp__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__help,info)
                cmd="dkp__subcmd__help__subcmd__info"
                ;;
            dkp__subcmd__help,init)
                cmd="dkp__subcmd__help__subcmd__init"
                ;;
            dkp__subcmd__help,inject)
                cmd="dkp__subcmd__help__subcmd__inject"
                ;;
            dkp__subcmd__help,install)
                cmd="dkp__subcmd__help__subcmd__install"
                ;;
            dkp__subcmd__help,keygen)
                cmd="dkp__subcmd__help__subcmd__keygen"
                ;;
            dkp__subcmd__help,l10n)
                cmd="dkp__subcmd__help__subcmd__l10n"
                ;;
            dkp__subcmd__help,list)
                cmd="dkp__subcmd__help__subcmd__list"
                ;;
            dkp__subcmd__help,mcp-manifest)
                cmd="dkp__subcmd__help__subcmd__mcp__subcmd__manifest"
                ;;
            dkp__subcmd__help,new)
                cmd="dkp__subcmd__help__subcmd__new"
                ;;
            dkp__subcmd__help,okf)
                cmd="dkp__subcmd__help__subcmd__okf"
                ;;
            dkp__subcmd__help,procedures)
                cmd="dkp__subcmd__help__subcmd__procedures"
                ;;
            dkp__subcmd__help,prompt)
                cmd="dkp__subcmd__help__subcmd__prompt"
                ;;
            dkp__subcmd__help,publish)
                cmd="dkp__subcmd__help__subcmd__publish"
                ;;
            dkp__subcmd__help,registry)
                cmd="dkp__subcmd__help__subcmd__registry"
                ;;
            dkp__subcmd__help,release-check)
                cmd="dkp__subcmd__help__subcmd__release__subcmd__check"
                ;;
            dkp__subcmd__help,review)
                cmd="dkp__subcmd__help__subcmd__review"
                ;;
            dkp__subcmd__help,rights)
                cmd="dkp__subcmd__help__subcmd__rights"
                ;;
            dkp__subcmd__help,run)
                cmd="dkp__subcmd__help__subcmd__run"
                ;;
            dkp__subcmd__help,search)
                cmd="dkp__subcmd__help__subcmd__search"
                ;;
            dkp__subcmd__help,serve)
                cmd="dkp__subcmd__help__subcmd__serve"
                ;;
            dkp__subcmd__help,sign)
                cmd="dkp__subcmd__help__subcmd__sign"
                ;;
            dkp__subcmd__help,skills)
                cmd="dkp__subcmd__help__subcmd__skills"
                ;;
            dkp__subcmd__help,tui)
                cmd="dkp__subcmd__help__subcmd__tui"
                ;;
            dkp__subcmd__help,uninstall)
                cmd="dkp__subcmd__help__subcmd__uninstall"
                ;;
            dkp__subcmd__help,update)
                cmd="dkp__subcmd__help__subcmd__update"
                ;;
            dkp__subcmd__help,validate)
                cmd="dkp__subcmd__help__subcmd__validate"
                ;;
            dkp__subcmd__help,webui)
                cmd="dkp__subcmd__help__subcmd__webui"
                ;;
            dkp__subcmd__help,yank)
                cmd="dkp__subcmd__help__subcmd__yank"
                ;;
            dkp__subcmd__help__subcmd__cross__subcmd__refs,list)
                cmd="dkp__subcmd__help__subcmd__cross__subcmd__refs__subcmd__list"
                ;;
            dkp__subcmd__help__subcmd__cross__subcmd__refs,validate)
                cmd="dkp__subcmd__help__subcmd__cross__subcmd__refs__subcmd__validate"
                ;;
            dkp__subcmd__help__subcmd__graph,list)
                cmd="dkp__subcmd__help__subcmd__graph__subcmd__list"
                ;;
            dkp__subcmd__help__subcmd__graph,stats)
                cmd="dkp__subcmd__help__subcmd__graph__subcmd__stats"
                ;;
            dkp__subcmd__help__subcmd__graph,validate)
                cmd="dkp__subcmd__help__subcmd__graph__subcmd__validate"
                ;;
            dkp__subcmd__help__subcmd__l10n,export)
                cmd="dkp__subcmd__help__subcmd__l10n__subcmd__export"
                ;;
            dkp__subcmd__help__subcmd__l10n,list)
                cmd="dkp__subcmd__help__subcmd__l10n__subcmd__list"
                ;;
            dkp__subcmd__help__subcmd__l10n,validate)
                cmd="dkp__subcmd__help__subcmd__l10n__subcmd__validate"
                ;;
            dkp__subcmd__help__subcmd__okf,browse)
                cmd="dkp__subcmd__help__subcmd__okf__subcmd__browse"
                ;;
            dkp__subcmd__help__subcmd__okf,export)
                cmd="dkp__subcmd__help__subcmd__okf__subcmd__export"
                ;;
            dkp__subcmd__help__subcmd__okf,links)
                cmd="dkp__subcmd__help__subcmd__okf__subcmd__links"
                ;;
            dkp__subcmd__help__subcmd__okf,stats)
                cmd="dkp__subcmd__help__subcmd__okf__subcmd__stats"
                ;;
            dkp__subcmd__help__subcmd__okf,validate)
                cmd="dkp__subcmd__help__subcmd__okf__subcmd__validate"
                ;;
            dkp__subcmd__help__subcmd__procedures,list)
                cmd="dkp__subcmd__help__subcmd__procedures__subcmd__list"
                ;;
            dkp__subcmd__help__subcmd__procedures,new)
                cmd="dkp__subcmd__help__subcmd__procedures__subcmd__new"
                ;;
            dkp__subcmd__help__subcmd__procedures,validate)
                cmd="dkp__subcmd__help__subcmd__procedures__subcmd__validate"
                ;;
            dkp__subcmd__help__subcmd__registry,keys)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__keys"
                ;;
            dkp__subcmd__help__subcmd__registry,login)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__login"
                ;;
            dkp__subcmd__help__subcmd__registry,logout)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__logout"
                ;;
            dkp__subcmd__help__subcmd__registry,pack)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__pack"
                ;;
            dkp__subcmd__help__subcmd__registry,register)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__register"
                ;;
            dkp__subcmd__help__subcmd__registry,token)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__token"
                ;;
            dkp__subcmd__help__subcmd__registry__subcmd__keys,add)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__keys__subcmd__add"
                ;;
            dkp__subcmd__help__subcmd__registry__subcmd__pack,access)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__access"
                ;;
            dkp__subcmd__help__subcmd__registry__subcmd__pack,grant)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__grant"
                ;;
            dkp__subcmd__help__subcmd__registry__subcmd__pack,revoke)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__revoke"
                ;;
            dkp__subcmd__help__subcmd__registry__subcmd__pack,set-visibility)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__set__subcmd__visibility"
                ;;
            dkp__subcmd__help__subcmd__registry__subcmd__pack,versions)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__versions"
                ;;
            dkp__subcmd__help__subcmd__registry__subcmd__token,rotate)
                cmd="dkp__subcmd__help__subcmd__registry__subcmd__token__subcmd__rotate"
                ;;
            dkp__subcmd__help__subcmd__rights,add-source)
                cmd="dkp__subcmd__help__subcmd__rights__subcmd__add__subcmd__source"
                ;;
            dkp__subcmd__help__subcmd__rights,check)
                cmd="dkp__subcmd__help__subcmd__rights__subcmd__check"
                ;;
            dkp__subcmd__help__subcmd__rights,report)
                cmd="dkp__subcmd__help__subcmd__rights__subcmd__report"
                ;;
            dkp__subcmd__help__subcmd__rights,status)
                cmd="dkp__subcmd__help__subcmd__rights__subcmd__status"
                ;;
            dkp__subcmd__help__subcmd__skills,list)
                cmd="dkp__subcmd__help__subcmd__skills__subcmd__list"
                ;;
            dkp__subcmd__help__subcmd__skills,show)
                cmd="dkp__subcmd__help__subcmd__skills__subcmd__show"
                ;;
            dkp__subcmd__help__subcmd__skills,validate)
                cmd="dkp__subcmd__help__subcmd__skills__subcmd__validate"
                ;;
            dkp__subcmd__l10n,export)
                cmd="dkp__subcmd__l10n__subcmd__export"
                ;;
            dkp__subcmd__l10n,help)
                cmd="dkp__subcmd__l10n__subcmd__help"
                ;;
            dkp__subcmd__l10n,list)
                cmd="dkp__subcmd__l10n__subcmd__list"
                ;;
            dkp__subcmd__l10n,validate)
                cmd="dkp__subcmd__l10n__subcmd__validate"
                ;;
            dkp__subcmd__l10n__subcmd__help,export)
                cmd="dkp__subcmd__l10n__subcmd__help__subcmd__export"
                ;;
            dkp__subcmd__l10n__subcmd__help,help)
                cmd="dkp__subcmd__l10n__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__l10n__subcmd__help,list)
                cmd="dkp__subcmd__l10n__subcmd__help__subcmd__list"
                ;;
            dkp__subcmd__l10n__subcmd__help,validate)
                cmd="dkp__subcmd__l10n__subcmd__help__subcmd__validate"
                ;;
            dkp__subcmd__okf,browse)
                cmd="dkp__subcmd__okf__subcmd__browse"
                ;;
            dkp__subcmd__okf,export)
                cmd="dkp__subcmd__okf__subcmd__export"
                ;;
            dkp__subcmd__okf,help)
                cmd="dkp__subcmd__okf__subcmd__help"
                ;;
            dkp__subcmd__okf,links)
                cmd="dkp__subcmd__okf__subcmd__links"
                ;;
            dkp__subcmd__okf,stats)
                cmd="dkp__subcmd__okf__subcmd__stats"
                ;;
            dkp__subcmd__okf,validate)
                cmd="dkp__subcmd__okf__subcmd__validate"
                ;;
            dkp__subcmd__okf__subcmd__help,browse)
                cmd="dkp__subcmd__okf__subcmd__help__subcmd__browse"
                ;;
            dkp__subcmd__okf__subcmd__help,export)
                cmd="dkp__subcmd__okf__subcmd__help__subcmd__export"
                ;;
            dkp__subcmd__okf__subcmd__help,help)
                cmd="dkp__subcmd__okf__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__okf__subcmd__help,links)
                cmd="dkp__subcmd__okf__subcmd__help__subcmd__links"
                ;;
            dkp__subcmd__okf__subcmd__help,stats)
                cmd="dkp__subcmd__okf__subcmd__help__subcmd__stats"
                ;;
            dkp__subcmd__okf__subcmd__help,validate)
                cmd="dkp__subcmd__okf__subcmd__help__subcmd__validate"
                ;;
            dkp__subcmd__procedures,help)
                cmd="dkp__subcmd__procedures__subcmd__help"
                ;;
            dkp__subcmd__procedures,list)
                cmd="dkp__subcmd__procedures__subcmd__list"
                ;;
            dkp__subcmd__procedures,new)
                cmd="dkp__subcmd__procedures__subcmd__new"
                ;;
            dkp__subcmd__procedures,validate)
                cmd="dkp__subcmd__procedures__subcmd__validate"
                ;;
            dkp__subcmd__procedures__subcmd__help,help)
                cmd="dkp__subcmd__procedures__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__procedures__subcmd__help,list)
                cmd="dkp__subcmd__procedures__subcmd__help__subcmd__list"
                ;;
            dkp__subcmd__procedures__subcmd__help,new)
                cmd="dkp__subcmd__procedures__subcmd__help__subcmd__new"
                ;;
            dkp__subcmd__procedures__subcmd__help,validate)
                cmd="dkp__subcmd__procedures__subcmd__help__subcmd__validate"
                ;;
            dkp__subcmd__registry,help)
                cmd="dkp__subcmd__registry__subcmd__help"
                ;;
            dkp__subcmd__registry,keys)
                cmd="dkp__subcmd__registry__subcmd__keys"
                ;;
            dkp__subcmd__registry,login)
                cmd="dkp__subcmd__registry__subcmd__login"
                ;;
            dkp__subcmd__registry,logout)
                cmd="dkp__subcmd__registry__subcmd__logout"
                ;;
            dkp__subcmd__registry,pack)
                cmd="dkp__subcmd__registry__subcmd__pack"
                ;;
            dkp__subcmd__registry,register)
                cmd="dkp__subcmd__registry__subcmd__register"
                ;;
            dkp__subcmd__registry,token)
                cmd="dkp__subcmd__registry__subcmd__token"
                ;;
            dkp__subcmd__registry__subcmd__help,help)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__registry__subcmd__help,keys)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__keys"
                ;;
            dkp__subcmd__registry__subcmd__help,login)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__login"
                ;;
            dkp__subcmd__registry__subcmd__help,logout)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__logout"
                ;;
            dkp__subcmd__registry__subcmd__help,pack)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__pack"
                ;;
            dkp__subcmd__registry__subcmd__help,register)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__register"
                ;;
            dkp__subcmd__registry__subcmd__help,token)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__token"
                ;;
            dkp__subcmd__registry__subcmd__help__subcmd__keys,add)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__keys__subcmd__add"
                ;;
            dkp__subcmd__registry__subcmd__help__subcmd__pack,access)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__access"
                ;;
            dkp__subcmd__registry__subcmd__help__subcmd__pack,grant)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__grant"
                ;;
            dkp__subcmd__registry__subcmd__help__subcmd__pack,revoke)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__revoke"
                ;;
            dkp__subcmd__registry__subcmd__help__subcmd__pack,set-visibility)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__set__subcmd__visibility"
                ;;
            dkp__subcmd__registry__subcmd__help__subcmd__pack,versions)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__versions"
                ;;
            dkp__subcmd__registry__subcmd__help__subcmd__token,rotate)
                cmd="dkp__subcmd__registry__subcmd__help__subcmd__token__subcmd__rotate"
                ;;
            dkp__subcmd__registry__subcmd__keys,add)
                cmd="dkp__subcmd__registry__subcmd__keys__subcmd__add"
                ;;
            dkp__subcmd__registry__subcmd__keys,help)
                cmd="dkp__subcmd__registry__subcmd__keys__subcmd__help"
                ;;
            dkp__subcmd__registry__subcmd__keys__subcmd__help,add)
                cmd="dkp__subcmd__registry__subcmd__keys__subcmd__help__subcmd__add"
                ;;
            dkp__subcmd__registry__subcmd__keys__subcmd__help,help)
                cmd="dkp__subcmd__registry__subcmd__keys__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__registry__subcmd__pack,access)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__access"
                ;;
            dkp__subcmd__registry__subcmd__pack,grant)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__grant"
                ;;
            dkp__subcmd__registry__subcmd__pack,help)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__help"
                ;;
            dkp__subcmd__registry__subcmd__pack,revoke)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__revoke"
                ;;
            dkp__subcmd__registry__subcmd__pack,set-visibility)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__set__subcmd__visibility"
                ;;
            dkp__subcmd__registry__subcmd__pack,versions)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__versions"
                ;;
            dkp__subcmd__registry__subcmd__pack__subcmd__help,access)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__access"
                ;;
            dkp__subcmd__registry__subcmd__pack__subcmd__help,grant)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__grant"
                ;;
            dkp__subcmd__registry__subcmd__pack__subcmd__help,help)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__registry__subcmd__pack__subcmd__help,revoke)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__revoke"
                ;;
            dkp__subcmd__registry__subcmd__pack__subcmd__help,set-visibility)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__set__subcmd__visibility"
                ;;
            dkp__subcmd__registry__subcmd__pack__subcmd__help,versions)
                cmd="dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__versions"
                ;;
            dkp__subcmd__registry__subcmd__token,help)
                cmd="dkp__subcmd__registry__subcmd__token__subcmd__help"
                ;;
            dkp__subcmd__registry__subcmd__token,rotate)
                cmd="dkp__subcmd__registry__subcmd__token__subcmd__rotate"
                ;;
            dkp__subcmd__registry__subcmd__token__subcmd__help,help)
                cmd="dkp__subcmd__registry__subcmd__token__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__registry__subcmd__token__subcmd__help,rotate)
                cmd="dkp__subcmd__registry__subcmd__token__subcmd__help__subcmd__rotate"
                ;;
            dkp__subcmd__rights,add-source)
                cmd="dkp__subcmd__rights__subcmd__add__subcmd__source"
                ;;
            dkp__subcmd__rights,check)
                cmd="dkp__subcmd__rights__subcmd__check"
                ;;
            dkp__subcmd__rights,help)
                cmd="dkp__subcmd__rights__subcmd__help"
                ;;
            dkp__subcmd__rights,report)
                cmd="dkp__subcmd__rights__subcmd__report"
                ;;
            dkp__subcmd__rights,status)
                cmd="dkp__subcmd__rights__subcmd__status"
                ;;
            dkp__subcmd__rights__subcmd__help,add-source)
                cmd="dkp__subcmd__rights__subcmd__help__subcmd__add__subcmd__source"
                ;;
            dkp__subcmd__rights__subcmd__help,check)
                cmd="dkp__subcmd__rights__subcmd__help__subcmd__check"
                ;;
            dkp__subcmd__rights__subcmd__help,help)
                cmd="dkp__subcmd__rights__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__rights__subcmd__help,report)
                cmd="dkp__subcmd__rights__subcmd__help__subcmd__report"
                ;;
            dkp__subcmd__rights__subcmd__help,status)
                cmd="dkp__subcmd__rights__subcmd__help__subcmd__status"
                ;;
            dkp__subcmd__skills,help)
                cmd="dkp__subcmd__skills__subcmd__help"
                ;;
            dkp__subcmd__skills,list)
                cmd="dkp__subcmd__skills__subcmd__list"
                ;;
            dkp__subcmd__skills,show)
                cmd="dkp__subcmd__skills__subcmd__show"
                ;;
            dkp__subcmd__skills,validate)
                cmd="dkp__subcmd__skills__subcmd__validate"
                ;;
            dkp__subcmd__skills__subcmd__help,help)
                cmd="dkp__subcmd__skills__subcmd__help__subcmd__help"
                ;;
            dkp__subcmd__skills__subcmd__help,list)
                cmd="dkp__subcmd__skills__subcmd__help__subcmd__list"
                ;;
            dkp__subcmd__skills__subcmd__help,show)
                cmd="dkp__subcmd__skills__subcmd__help__subcmd__show"
                ;;
            dkp__subcmd__skills__subcmd__help,validate)
                cmd="dkp__subcmd__skills__subcmd__help__subcmd__validate"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        dkp)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__build)
            opts="-q -v -h -V --format --out --no-human --gen-mcp-manifest --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__chunk)
            opts="-q -v -h -V --top --min-confidence --output --quiet --verbose --audience --help --version <PACK> <QUERY>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --top)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-confidence)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__cross__subcmd__refs)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK> list validate help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__cross__subcmd__refs__subcmd__help)
            opts="list validate help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__cross__subcmd__refs__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__cross__subcmd__refs__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__cross__subcmd__refs__subcmd__help__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__cross__subcmd__refs__subcmd__list)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__cross__subcmd__refs__subcmd__validate)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__diff)
            opts="-q -v -h -V --type --threshold --output --quiet --verbose --audience --help --version <PACK_A> <PACK_B>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__eval)
            opts="-q -v -h -V --provider --model --base-url --api-key --pairs --baseline-only --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --provider)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --model)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --base-url)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --api-key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pairs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__export)
            opts="-q -v -h -V --out --output --quiet --verbose --audience --help --version <PACK> <FORMAT>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__fix)
            opts="-q -v -h -V --api-key --base-url --model --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --api-key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --base-url)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --model)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__generate)
            opts="-q -v -h -V --overwrite --api-key --base-url --model --output --quiet --verbose --audience --help --version <PACK> [ASSET]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --api-key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --base-url)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --model)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__get)
            opts="-q -v -h -V --by-id --output --quiet --verbose --audience --help --version <PACK> <TYPE> [ID]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK> stats validate list help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph__subcmd__help)
            opts="stats validate list help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph__subcmd__help__subcmd__stats)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph__subcmd__help__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph__subcmd__list)
            opts="-q -v -h -V --type --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph__subcmd__stats)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__graph__subcmd__validate)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help)
            opts="init info list validate search get inject export okf chunk eval prompt diff build release-check rights mcp-manifest serve tui webui run procedures graph cross-refs skills l10n new generate fix review keygen sign install uninstall update publish yank registry help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__chunk)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__cross__subcmd__refs)
            opts="list validate"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__cross__subcmd__refs__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__cross__subcmd__refs__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__eval)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__fix)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__generate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__graph)
            opts="stats validate list"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__graph__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__graph__subcmd__stats)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__graph__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__inject)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__keygen)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__l10n)
            opts="list validate export"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__l10n__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__l10n__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__l10n__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__mcp__subcmd__manifest)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__new)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__okf)
            opts="export validate stats links browse"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__okf__subcmd__browse)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__okf__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__okf__subcmd__links)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__okf__subcmd__stats)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__okf__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__procedures)
            opts="list validate new"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__procedures__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__procedures__subcmd__new)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__procedures__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__prompt)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__publish)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry)
            opts="register login logout token keys pack"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__keys)
            opts="add"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__keys__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__login)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__logout)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__pack)
            opts="versions set-visibility grant revoke access"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__access)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__grant)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__revoke)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__set__subcmd__visibility)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__pack__subcmd__versions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__register)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__token)
            opts="rotate"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__registry__subcmd__token__subcmd__rotate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__release__subcmd__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__review)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__rights)
            opts="status check add-source report"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__rights__subcmd__add__subcmd__source)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__rights__subcmd__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__rights__subcmd__report)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__rights__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__run)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__search)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__serve)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__sign)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__skills)
            opts="list validate show"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__skills__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__skills__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__skills__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__tui)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__uninstall)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__webui)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__help__subcmd__yank)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__info)
            opts="-q -v -h -V --check --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__init)
            opts="-q -v -h -V --domain --out --extras --force --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__inject)
            opts="-q -v -h -V --scope --format --count-tokens --max-tokens --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --scope)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__install)
            opts="-g -q -v -h -V --global --dest --registry --token --no-verify --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --dest)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --token)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__keygen)
            opts="-q -v -h -V --out --force --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK> list validate export help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n__subcmd__export)
            opts="-q -v -h -V --out --output --quiet --verbose --audience --help --version <LOCALE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n__subcmd__help)
            opts="list validate export help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n__subcmd__help__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n__subcmd__help__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n__subcmd__list)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__l10n__subcmd__validate)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__list)
            opts="-q -v -h -V --domain --tier --output --quiet --verbose --audience --help --version <ROOT>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --tier)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__mcp__subcmd__manifest)
            opts="-q -v -h -V --dry-run --out --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__new)
            opts="-q -v -h -V --domain --dir --api-key --base-url --model --overwrite --skip-validate --skip-package --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --api-key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --base-url)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --model)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK> export validate stats links browse help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__browse)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__export)
            opts="-q -v -h -V --out --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__help)
            opts="export validate stats links browse help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__help__subcmd__browse)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__help__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__help__subcmd__links)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__help__subcmd__stats)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__help__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__links)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__stats)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__okf__subcmd__validate)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version list validate new help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures__subcmd__help)
            opts="list validate new help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures__subcmd__help__subcmd__new)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures__subcmd__help__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures__subcmd__list)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures__subcmd__new)
            opts="-q -v -h -V --title --description --lang --output --quiet --verbose --audience --help --version <PACK> <ID>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --title)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --description)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --lang)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__procedures__subcmd__validate)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__prompt)
            opts="-q -v -h -V --provider --model --api-key --max-tokens --scope --output --quiet --verbose --audience --help --version <PACK> [QUESTION]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --provider)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --model)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --api-key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --scope)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__publish)
            opts="-q -v -h -V --url --private --registry --token --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --url)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --token)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version register login logout token keys pack help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help)
            opts="register login logout token keys pack help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__keys)
            opts="add"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__keys__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__login)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__logout)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__pack)
            opts="versions set-visibility grant revoke access"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__access)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__grant)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__revoke)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__set__subcmd__visibility)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__pack__subcmd__versions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__register)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__token)
            opts="rotate"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__help__subcmd__token__subcmd__rotate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__keys)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version add help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__keys__subcmd__add)
            opts="-q -v -h -V --key --registry --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__keys__subcmd__help)
            opts="add help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__keys__subcmd__help__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__keys__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__login)
            opts="-q -v -h -V --email --registry --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --email)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__logout)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version versions set-visibility grant revoke access help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__access)
            opts="-q -v -h -V --registry --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__grant)
            opts="-q -v -h -V --to --registry --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --to)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__help)
            opts="versions set-visibility grant revoke access help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__access)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__grant)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__revoke)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__set__subcmd__visibility)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__help__subcmd__versions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__revoke)
            opts="-q -v -h -V --from --registry --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --from)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__set__subcmd__visibility)
            opts="-q -v -h -V --registry --output --quiet --verbose --audience --help --version <NAME> <VISIBILITY>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__pack__subcmd__versions)
            opts="-q -v -h -V --registry --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__register)
            opts="-q -v -h -V --email --registry --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --email)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__token)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version rotate help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__token__subcmd__help)
            opts="rotate help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__token__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__token__subcmd__help__subcmd__rotate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__registry__subcmd__token__subcmd__rotate)
            opts="-q -v -h -V --registry --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__release__subcmd__check)
            opts="-q -v -h -V --strict --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__review)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK> status check add-source report help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__add__subcmd__source)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__check)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__help)
            opts="status check add-source report help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__help__subcmd__add__subcmd__source)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__help__subcmd__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__help__subcmd__report)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__report)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__rights__subcmd__status)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__run)
            opts="-q -v -h -V --input --timeout-ms --allow-unsigned --output --quiet --verbose --audience --help --version <PACK> <PROCEDURE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --input)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --timeout-ms)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__search)
            opts="-q -v -h -V --registry --type --limit --domain --conformance --output --quiet --verbose --audience --help --version [PACK] [QUERY]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --domain)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --conformance)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__serve)
            opts="-q -v -h -V --transport --port --readonly --auth-token --log-level --allow-unsigned --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --transport)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --auth-token)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__sign)
            opts="-q -v -h -V --key --out --output --quiet --verbose --audience --help --version <ARCHIVE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK> list validate show help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills__subcmd__help)
            opts="list validate show help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills__subcmd__help__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills__subcmd__help__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills__subcmd__list)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills__subcmd__show)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__skills__subcmd__validate)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__tui)
            opts="-q -v -h -V --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__uninstall)
            opts="-g -q -v -h -V --global --dest --all-versions --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --dest)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__update)
            opts="-q -v -h -V --registry --token --output --quiet --verbose --audience --help --version [NAME]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --token)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__validate)
            opts="-q -v -h -V --strict --gate --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --gate)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__webui)
            opts="-q -v -h -V --port --no-open --output --quiet --verbose --audience --help --version <PACK>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        dkp__subcmd__yank)
            opts="-q -v -h -V --reason --registry --token --output --quiet --verbose --audience --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --reason)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --registry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --token)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -W "plain table json" -- "${cur}"))
                    return 0
                    ;;
                --audience)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _dkp -o nosort -o bashdefault -o default dkp
else
    complete -F _dkp -o bashdefault -o default dkp
fi
