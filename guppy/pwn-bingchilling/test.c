#include <stdio.h>
#include <sys/syscall.h>

// void shell_asm() {
// 	__asm__ (
// 		"addi.d $a7, $zero, 221\n"
// 		"move $a2, $zero\n"
// 		"move $a1, $zero\n"
// 		//"addi.d $a3, $zero, 0x68732f6e69622f2f\n"
// 		"lu12i.w $a3, 0x6e696\n"
// 		"ori $a3, $a3, 0x22f\n"
// 		"lu32i.d $a3, -494801\n"
// 		"lu52i.d $a3, $a3, 6\n"
// 		"st.d $a3, $sp, 0\n"
// 		"move $a0, $sp\n"
// 		"syscall 0"
// 	);	
// }

// void shell() {
// 	syscall(SYS_execve, "/bin/sh", NULL, NULL);
// 	unsigned long long a = 0x68732f6e69622f;
// }

int main() {
	char buf[64];
	printf("请输入您的名称：\n");
	gets(buf);
	printf("你好，%s。欢迎光临！\n", buf);
}
