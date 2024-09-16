import { z } from "zod";

export const createCampaignFormSchema = z.object({
  title: z.string().min(2, "title is too short").max(50, "title is too long"),
  description: z
    .string()
    .min(2, "description is too short")
    .max(500, "description is too long"),
  target: z
    .number()
    .int("target must be an integer")
    .positive("target must be positive")
    .gte(1, "target must be at least 1"),
  duration: z
    .number()
    .int("duration must be an integer")
    .positive("duration must be positive")
    .gte(1, "duration must be at least 1"),
});
