_csview() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="csview"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        csview)
            opts="-H -n -t -d -s -p -i -h -V --no-headers --number --tsv --delimiter --style --padding --indent --sniff --header-align --body-align --help --version [FILE]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --delimiter)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --style)
                    COMPREPLY=($(compgen -W "none ascii ascii2 sharp rounded reinforced markdown grid" -- "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -W "none ascii ascii2 sharp rounded reinforced markdown grid" -- "${cur}"))
                    return 0
                    ;;
                --padding)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --indent)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --sniff)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --header-align)
                    COMPREPLY=($(compgen -W "left center right" -- "${cur}"))
                    return 0
                    ;;
                --body-align)
                    COMPREPLY=($(compgen -W "left center right" -- "${cur}"))
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

complete -F _csview -o bashdefault -o default csview
