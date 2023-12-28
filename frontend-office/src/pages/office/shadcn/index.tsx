import { Button } from "@/components/ui/button";
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "@/components/ui/drawer"

export default function Shadcn() {
  return (
    <>
      <h1>Hello there!</h1>
			<Button>Hello there</Button>
			<Drawer>
				<DrawerTrigger>
					<Button>
						Open
					</Button>
				</DrawerTrigger>
				<DrawerContent className="h-4/5 lg:px-96">
						<DrawerHeader>
							<DrawerTitle>Are you sure absolutely sure?</DrawerTitle>
							<DrawerDescription>This action cannot be undone.</DrawerDescription>
						</DrawerHeader>
						<DrawerFooter>
							<Button>Submit</Button>
							<DrawerClose>
								<Button variant="outline" className="w-full">Cancel</Button>
							</DrawerClose>
						</DrawerFooter>
				</DrawerContent>
			</Drawer>
    </>
  )
}