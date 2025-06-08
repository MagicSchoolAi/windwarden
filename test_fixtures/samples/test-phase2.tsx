import { cn } from "lib/utils";

function Button({ variant, size, className, ...props }) {
  return (
    <button
      className={cn(
        "p-4 flex items-center bg-blue-500 text-white font-semibold rounded-lg",
        variant === "outline" && "border-2 border-blue-500 bg-transparent text-blue-500",
        size === "sm" && "text-sm p-2",
        className
      )}
      {...props}
    />
  );
}

function Card() {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-lg font-bold mb-4">Card Title</h2>
      <Button 
        variant="outline" 
        size="sm"
        className={twMerge("mr-2 hover:bg-gray-100", "focus:ring-2 focus:ring-blue-300")}
      >
        Click me
      </Button>
    </div>
  );
}