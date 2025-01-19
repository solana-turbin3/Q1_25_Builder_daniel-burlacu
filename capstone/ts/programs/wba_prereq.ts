export type WbaPrereq = {
  version: "0.1.0",
  name: "wba_prereq",
  metadata: {
    address: "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1",
    name: "WBA Prerequisite Program", // Required field
    version: "0.1.0",                // Required field
    spec: string                      // Ensure 'spec' is always a string
  },
  address: "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1", // Retained
  instructions: [
    {
      name: "complete",
      discriminator: [0, 0, 0, 0, 0, 0, 0, 1], // Add discriminator
      accounts: [
        {
          name: "signer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "prereq",
          isMut: true,
          isSigner: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "github",
          type: "bytes",
        },
      ],
    },
    {
      name: "update",
      discriminator: [0, 0, 0, 0, 0, 0, 0, 2], // Add discriminator
      accounts: [
        {
          name: "signer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "prereq",
          isMut: true,
          isSigner: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "github",
          type: "bytes",
        },
      ],
    },
  ],
  accounts: [
    {
      name: "PrereqAccount",
      type: {
        kind: "struct",
        fields: [
          {
            name: "github",
            type: "bytes",
          },
          {
            name: "key",
            type: "publicKey",
          },
        ],
      },
    },
  ],
  errors: [
    {
      code: 6000,
      name: "InvalidGithubAccount",
      msg: "Invalid Github account",
    },
  ],
};

export const IDL: WbaPrereq = {
  version: "0.1.0",
  name: "wba_prereq",
  metadata: {
    address: "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1",
    name: "WBA Prerequisite Program", // Required field
    version: "0.1.0",                // Required field
    spec: "Some specification"        // Ensure 'spec' is a string
  },
  address: "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1",
  instructions: [
    {
      name: "complete",
      discriminator: [0, 0, 0, 0, 0, 0, 0, 1], // Add discriminator
      accounts: [
        {
          name: "signer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "prereq",
          isMut: true,
          isSigner: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "github",
          type: "bytes",
        },
      ],
    },
    {
      name: "update",
      discriminator: [0, 0, 0, 0, 0, 0, 0, 2], // Add discriminator
      accounts: [
        {
          name: "signer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "prereq",
          isMut: true,
          isSigner: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "github",
          type: "bytes",
        },
      ],
    },
  ],
  accounts: [
    {
      name: "PrereqAccount",
      type: {
        kind: "struct",
        fields: [
          {
            name: "github",
            type: "bytes",
          },
          {
            name: "key",
            type: "publicKey",
          },
        ],
      },
    },
  ],
  errors: [
    {
      code: 6000,
      name: "InvalidGithubAccount",
      msg: "Invalid Github account",
    },
  ],
};
