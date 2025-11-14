_smart_term() {
    local cur prev words cword
    _init_completion || return

    case $prev in
        smart-term)
            COMPREPLY=($(compgen -W "--help --version --ui" -- "$cur"))
            ;;
        --ui)
            COMPREPLY=($(compgen -W "true false" -- "$cur"))
            ;;
        *)
            COMPREPLY=($(compgen -W "help bash-help bash-quick history clear elevate privileges" -- "$cur"))
            ;;
    esac
}

complete -F _smart_term smart-term
