import { CreateCampaignFormField } from "@/@types/create-campaign.interfaces";

export const createCampaignFormFields: {
  id: number;
  name: CreateCampaignFormField;
  label: string;
  description: string;
}[] = [
  {
    id: 1,
    name: "title",
    label: "Title",
    description: "Title of your campaign",
  },
  {
    id: 2,
    name: "description",
    label: "Description",
    description: "Description of your campaign",
  },
  {
    id: 3,
    name: "target",
    label: "Target",
    description: "Target amount to be raised",
  },
  {
    id: 4,
    name: "duration",
    label: "Duration",
    description: "Duration of the campaign",
  },
];
