#[doc(hidden)]
#[macro_export]
macro_rules! __interrupt_vector {
    (INT0, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_1() {$b}};
    (INT1, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_2() {$b}};
    (INT2, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_3() {$b}};
    (INT3, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_4() {$b}};
    (INT6, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_7() {$b}};
    (USB_GEN, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_10() {$b}};
    (USB_COM, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_11() {$b}};
    (TIMER1_CAPT, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_16() {$b}};
    (TIMER1_COMPA, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_17() {$b}};
    (TIMER1_COMPB, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_18() {$b}};
    (TIMER1_COMPC, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_19() {$b}};
    (TIMER1_OVF, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_20() {$b}};
    (TIMER0_COMPA, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_21() {$b}};
    (TIMER0_COMPB, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_22() {$b}};
    (TIMER0_OVF, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_23() {$b}};
    (TIMER3_CAPT, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_31() {$b}};
    (TIMER3_COMPA, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_32() {$b}};
    (TIMER3_COMPB, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_33() {$b}};
    (TIMER3_COMPC, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_34() {$b}};
    (TIMER3_OVF, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_35() {$b}};
    (TIMER4_COMPA, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_38() {$b}};
    (TIMER4_COMPB, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_39() {$b}};
    (TIMER4_COMPD, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_40() {$b}};
    (TIMER4_OVF, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_41() {$b}};
    (TIMER4_FPF, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_42() {$b}};
}
