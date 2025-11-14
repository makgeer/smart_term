#compdef smart-term

_smart-term() {
    local state

    _arguments \
        '1: :->commands' \
        '*: :->args' && ret=0

    case $state in
        commands)
            _values "smart-term commands" \
                "--help[Show help]" \
                "--version[Show version]" \
                "--ui[Start in UI mode]" \
                "help[Show terminal help]" \
                "bash-help[Show Bash help]" \
                "bash-quick[Show quick Bash reference]"
            ;;
    esac
}

_smart-term "$@"
