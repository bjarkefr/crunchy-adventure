    TITLE "Led breather"
    SUBTITLE ""

    processor	12F675
    radix	DEC
    include	"p12f675.inc"
    __CONFIG	_CP_OFF & _CPD_OFF & _WDT_OFF & _PWRTE_ON & _BODEN_ON & _MCLRE_OFF & _FOSC_INTRCIO

PWM_LVL EQU 0x20
ON_TIME EQU 0x21

FADE_FROM EQU 0x22
FADE_TO EQU 0x23

MAIN    CODE

    bsf STATUS, RP0
    bcf TRISIO, TRISIO5
    
    bcf STATUS, RP0

    movlw 5
    movwf ON_TIME
    movlw 0
    movwf FADE_FROM
    movlw 15
    movwf FADE_TO
    call fade_up_led

main_fade_loop:

    movlw 19
    movwf ON_TIME
    movlw 25
    movwf FADE_TO
    call fade_up_led

    movlw 18
    movwf ON_TIME
    movlw 50
    movwf FADE_TO

    call fade_up_led

    movlw 14
    movwf ON_TIME
    movlw 75
    movwf FADE_TO

    call fade_up_led

    movlw 10
    movwf ON_TIME
    movlw 100
    movwf FADE_TO

    call fade_up_led

    movlw 7
    movwf ON_TIME
    movlw 125
    movwf FADE_TO

    call fade_up_led

    movlw 5
    movwf ON_TIME
    movlw 150
    movwf FADE_TO

    call fade_up_led

    movlw 4
    movwf ON_TIME
    movlw 255
    movwf FADE_TO

    call fade_up_led

    movlw 4
    movwf ON_TIME
    movlw 151
    movwf FADE_TO

    call fade_down_led

    movlw 5
    movwf ON_TIME
    movlw 126
    movwf FADE_TO

    call fade_down_led

    movlw 7
    movwf ON_TIME
    movlw 101
    movwf FADE_TO

    call fade_down_led

    movlw 10
    movwf ON_TIME
    movlw 76
    movwf FADE_TO

    call fade_down_led

    movlw 14
    movwf ON_TIME
    movlw 51
    movwf FADE_TO

    call fade_down_led

    movlw 18
    movwf ON_TIME
    movlw 26
    movwf FADE_TO

    call fade_down_led

    movlw 19
    movwf ON_TIME
    movlw 15
    movwf FADE_TO

    call fade_down_led

    goto main_fade_loop

fade_up_led:

    movf FADE_FROM, W

    call pwm_led

    incf FADE_FROM, F

    movf FADE_TO, W
    xorwf FADE_FROM, W
    
    btfss STATUS, Z
    goto fade_up_led

    return

fade_down_led:

    movf FADE_FROM, W

    call pwm_led

    decf FADE_FROM, F

    movf FADE_TO, W
    xorwf FADE_FROM, W
    
    btfss STATUS, Z
    goto fade_down_led

    return

pwm_led:

    addlw 1

pwm_led_loop:

    bsf GPIO, GPIO5
    movwf PWM_LVL

wait_1:
    decfsz PWM_LVL, F
    goto wait_1

    bcf GPIO, GPIO5
    movwf PWM_LVL
    comf PWM_LVL, F

wait_2:
    decfsz PWM_LVL, F
    goto wait_2

    decfsz ON_TIME, F
    goto pwm_led_loop
    
    return

    end
