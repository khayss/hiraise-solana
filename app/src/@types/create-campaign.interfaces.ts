import { createCampaignFormSchema } from "@/lib/schemas";
import { z } from "zod";

export type CreateCampaignForm = z.infer<typeof createCampaignFormSchema>;

export type CreateCampaignFormField =
  | "title"
  | "description"
  | "target"
  | "duration";
