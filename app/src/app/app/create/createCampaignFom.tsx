"use client";

import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { CreateCampaignForm } from "@/@types/create-campaign.interfaces";
import { createCampaignFormSchema } from "@/lib/schemas";
import { createCampaignFormFields } from "@/data/create-campaign.data";
import { CreateCampaignIcon } from "@/components/icons/common";

export function CampaignForm() {
  const form = useForm<CreateCampaignForm>({
    resolver: zodResolver(createCampaignFormSchema),
    defaultValues: {
      title: "",
      description: "",
      target: 0,
      duration: 0,
    },
  });

  const handleSubmit = (values: CreateCampaignForm) => {
    console.log(values);
  };

  return (
    <div className="w-full max-w-[540px] px-4 md:px-8 lg:px-12 xl:px-16 py-16 flex flex-col items-center text-center gap-8">
      <div className="space-y-4">
        <h3 className="text-xl font-medium">Create Campaign</h3>
        <p className="text-sm">
          Lorem ipsum dolor sit amet consectetur adipisicing elit. Earum
          architecto vitae quae esse placeat consequatur officia facilis.
          Sapiente maiores, dolores sunt necessitatibus deleniti beatae
          voluptatem itaque provident ipsum nihil dicta.
        </p>
      </div>
      <Form {...form}>
        <form
          action=""
          onSubmit={form.handleSubmit(handleSubmit)}
          className="space-y-8 w-full"
        >
          {createCampaignFormFields.map((formField) => (
            <FormField
              key={formField.id}
              control={form.control}
              name={formField.name}
              render={({ field }) => (
                <FormItem className="flex flex-col items-start">
                  <FormLabel>{formField.label}</FormLabel>
                  <FormControl>
                    <Input placeholder="shadcn" {...field} />
                  </FormControl>
                  <FormDescription>{formField.description}</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          ))}
          <Button
            type="submit"
            className="w-full flex items-center justify-center gap-2"
          >
            <span>Create</span>
            <span>
              <CreateCampaignIcon width="20px" />
            </span>
          </Button>
        </form>
      </Form>
    </div>
  );
}
