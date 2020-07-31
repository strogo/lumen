#include "lumen/mlir/MLIR.h"

#include "lumen/EIR/IR/EIRDialect.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Dialect/StandardOps/IR/Ops.h"
#include "mlir/IR/MLIRContext.h"
#include "llvm/IR/LLVMContext.h"

using ::mlir::MLIRContext;
using ::llvm::LLVMContext;
using ::llvm::unwrap;

extern "C" void MLIRRegisterDialects(MLIRContextRef mlirCtx, LLVMContextRef llvmCtx) {
  MLIRContext *mlirContext = unwrap(mlirCtx);
  LLVMContext *llvmContext = unwrap(llvmCtx);

  // Register the LLVM and EIR dialects with MLIR, providing them
  // with the current thread's LLVMContext.
  //
  // NOTE: The dialect constructors internally call registerDialect,
  // which moves ownership of the dialect objects to the MLIRContext, 
  // so we don't have to manage them ourselves.
  auto *stdDialect = new mlir::StandardOpsDialect(mlirContext);
  auto *llvmDialect = new mlir::LLVM::LLVMDialect(mlirContext, llvmContext);
  auto *eirDialect = new lumen::eir::eirDialect(mlirContext);
}