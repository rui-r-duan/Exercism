	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.globl	__ZN11eliuds_eggs9egg_count17h00c418676acd245dE
	.p2align	2
__ZN11eliuds_eggs9egg_count17h00c418676acd245dE:
	.cfi_startproc
	fmov	s0, w0
	cnt.8b	v0, v0
	uaddlv.8b	h0, v0
	fmov	w0, s0
	ret
	.cfi_endproc

.subsections_via_symbols
