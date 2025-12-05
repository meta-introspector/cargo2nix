#include <stdio.h>
#include <dlfcn.h>

int main() {
    printf("🔬 PROOF: Testing Monster Group Symbiotic Compiler Output\n");
    printf("=========================================================\n");
    
    // Load the Monster Group compiled library
    void* handle = dlopen("./libmonster.so", RTLD_LAZY);
    if (!handle) {
        printf("❌ Failed to load libmonster.so: %s\n", dlerror());
        return 1;
    }
    
    // Get the monster_main function
    int (*monster_main)() = dlsym(handle, "monster_main");
    if (!monster_main) {
        printf("❌ Failed to find monster_main: %s\n", dlerror());
        dlclose(handle);
        return 1;
    }
    
    // Call the Monster Group compiled function
    printf("📞 Calling monster_main() compiled by symbiotic compiler...\n");
    int result = monster_main();
    
    printf("✅ Result: %d\n", result);
    
    if (result == 71) {
        printf("🎉 SUCCESS: Monster Group sentinel value 71 returned!\n");
        printf("👑 PROOF COMPLETE: Symbiotic compiler generated working code!\n");
    } else {
        printf("❌ Unexpected result: expected 71, got %d\n", result);
    }
    
    dlclose(handle);
    return result == 71 ? 0 : 1;
}
