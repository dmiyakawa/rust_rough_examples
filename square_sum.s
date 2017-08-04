	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
__ZN10square_sum4main17hf31d1cbe24331a96E:
	.cfi_startproc
	xorl	%eax, %eax
	xorl	%ecx, %ecx
	jmp	LBB0_1
LBB0_3:
	leaq	-1(%rax), %rdx
	imulq	%rdx, %rdx
	addq	%rdx, %rcx
	.p2align	4, 0x90
LBB0_1:
	cmpq	$9, %rax
	jg	LBB0_4
	movl	%eax, %edx
	incq	%rax
	testb	$1, %dl
	jne	LBB0_1
	jmp	LBB0_3
LBB0_4:
	pushq	%rbp
Ltmp0:
	.cfi_def_cfa_offset 16
Ltmp1:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp2:
	.cfi_def_cfa_register %rbp
	subq	$80, %rsp
	movq	%rcx, -8(%rbp)
	leaq	-8(%rbp), %rax
	movq	%rax, -24(%rbp)
	movq	__ZN4core3fmt3num54_$LT$impl$u20$core..fmt..Display$u20$for$u20$isize$GT$3fmt17h7d8cff6db5a75a13E@GOTPCREL(%rip), %rax
	movq	%rax, -16(%rbp)
	leaq	_ref.7(%rip), %rax
	movq	%rax, -72(%rbp)
	movq	$2, -64(%rbp)
	movq	$0, -56(%rbp)
	leaq	-24(%rbp), %rax
	movq	%rax, -40(%rbp)
	movq	$1, -32(%rbp)
	leaq	-72(%rbp), %rdi
	callq	__ZN3std2io5stdio6_print17h2a346cba6dd9990aE
	addq	$80, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:
	.cfi_startproc
	pushq	%rbp
Ltmp3:
	.cfi_def_cfa_offset 16
Ltmp4:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp5:
	.cfi_def_cfa_register %rbp
	movq	%rsi, %rax
	movq	%rdi, %rcx
	leaq	__ZN10square_sum4main17hf31d1cbe24331a96E(%rip), %rdi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	popq	%rbp
	jmp	__ZN3std2rt10lang_start17ha1f4f04346e51ef5E
	.cfi_endproc

	.section	__TEXT,__const
_str.5:
	.byte	0

_str.6:
	.byte	10

	.section	__DATA,__const
	.p2align	3
_ref.7:
	.quad	_str.5
	.quad	0
	.quad	_str.6
	.quad	1


.subsections_via_symbols
