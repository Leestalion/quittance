<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useLeasesStore } from '../stores/leases'
import { usePropertiesStore } from '../stores/properties'
import { useTenantsStore } from '../stores/tenants'
import { useReceiptsStore } from '../stores/receipts'
import { useAuthStore } from '../stores/auth'
import { useOrganizationsStore } from '../stores/organizations'
import LeasePreview from '../components/LeasePreview.vue'
import type { LeaseData, FurnitureSet, FurnitureSetWithItems, Lease } from '../types'
import { buildComplianceWarnings } from '../utils/leaseCompliance'

const route = useRoute()
const router = useRouter()
const leasesStore = useLeasesStore()
const propertiesStore = usePropertiesStore()
const tenantsStore = useTenantsStore()
const receiptsStore = useReceiptsStore()
const authStore = useAuthStore()
const organizationsStore = useOrganizationsStore()

const loading = ref(false)
const error = ref<string | null>(null)
const showPreview = ref(false)
const isNewlyCreated = ref(false)
const generatedLeaseId = ref<string | null>(null)
const generatedComplianceStatus = ref<string | null>(null)
const furnitureSets = ref<FurnitureSet[]>([])
const selectedFurnitureSets = ref<FurnitureSetWithItems[]>([])

// Form data
const formData = ref({
  tenant_ids: [] as string[],
  start_date: new Date().toISOString().split('T')[0],
  duration_months: 12,
  lease_kind: 'standard' as 'standard' | 'student',
  is_colocation: false,
  destination: 'habitation' as 'habitation' | 'mixte_professionnel_habitation',
  monthly_rent: 0,
  charges: 0,
  deposit: 0,
  rent_revision: true,
  habitable_surface: 0,
  main_room_count: 1,
  heating_mode: 'individuel' as 'individuel' | 'collectif',
  hot_water_mode: 'individuelle' as 'individuelle' | 'collective',
  dpe_class: 'D' as 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G',
  is_dom_tom: false,
  energy_cost_annual: '',
  energy_cost_year: new Date().getFullYear(),
  rent_payment_frequency: 'mensuel' as 'mensuel' | 'trimestriel',
  rent_payment_timing: 'a_echoir' as 'a_echoir' | 'a_terme_echu',
  rent_payment_period: 'le 1er de chaque mois',
  rent_controlled: false,
  reference_rent: 0,
  reference_rent_majorated: 0,
  rent_complement: 0,
  rent_complement_justification: '',
  previous_tenant_departure_date: '',
  previous_tenant_last_rent: 0,
  professional_mandate: false,
  agency_fee_tenant: 0,
  agency_fee_landlord: 0,
  custom_clauses: '',
  inventory_date: '',
  private_room_label: '',
  shared_areas_text: '',
  furniture_set_ids: [] as string[],
  furniture_inventory: '',
  erp: '',
  home_insurance: '',
  legal_notice_provided: true,
  annex_entry_inventory_provided: true,
  annex_furniture_inventory_provided: true,
  annex_dpe_provided: true,
  annex_erp_provided: true,
  annex_home_insurance_provided: true,
  // Legal completeness: Section II characterisation
  identifiant_fiscal: '',
  habitat_type: 'collectif' as 'collectif' | 'individuel',
  regime_juridique: 'copropriete' as 'monopropriete' | 'copropriete',
  construction_period: '1989_2005' as 'avant_1949' | '1949_1974' | '1975_1989' | '1989_2005' | 'depuis_2005',
  // Property-fact flags driving annex gating
  electrical_installation_over_15y: false,
  gas_installation_over_15y: false,
  in_risk_zone: false,
  annex_lead_provided: false,
  annex_electrical_provided: false,
  annex_gas_provided: false,
  annex_risk_provided: false,
})

const propertyId = computed(() => route.params.propertyId as string)
const property = computed(() => propertiesStore.getPropertyById(propertyId.value))
const allTenants = computed(() => tenantsStore.tenants)
const primaryTenant = computed(() =>
  tenantsStore.getTenantById(formData.value.tenant_ids[0] ?? '')
)
const isEditMode = computed(() => generatedLeaseId.value !== null)

const complianceWarnings = computed(() => {
  return buildComplianceWarnings(
    { ...formData.value, tenant_count: formData.value.tenant_ids.length },
    property.value?.furnished ?? false,
  )
})

function applyLeaseToForm(lease: Lease) {
  formData.value = {
    tenant_ids: lease.tenant_ids && lease.tenant_ids.length > 0 ? [...lease.tenant_ids] : [lease.tenant_id],
    start_date: lease.start_date,
    duration_months: lease.duration_months,
    lease_kind: lease.lease_kind,
    is_colocation: lease.is_colocation,
    destination: lease.destination,
    monthly_rent: Number(lease.monthly_rent),
    charges: Number(lease.charges),
    deposit: Number(lease.deposit),
    rent_revision: lease.rent_revision,
    habitable_surface: Number(lease.habitable_surface ?? property.value?.surface_area ?? 0),
    main_room_count: Number(lease.main_room_count ?? property.value?.rooms ?? 1),
    heating_mode: lease.heating_mode ?? 'individuel',
    hot_water_mode: lease.hot_water_mode ?? 'individuelle',
    dpe_class: lease.dpe_class ?? 'D',
    is_dom_tom: lease.is_dom_tom,
    energy_cost_annual: lease.energy_cost_annual ?? '',
    energy_cost_year: lease.energy_cost_year ?? new Date().getFullYear(),
    rent_payment_frequency: lease.rent_payment_frequency,
    rent_payment_timing: lease.rent_payment_timing,
    rent_payment_period: lease.rent_payment_period ?? 'le 1er de chaque mois',
    rent_controlled: lease.rent_controlled,
    reference_rent: Number(lease.reference_rent ?? 0),
    reference_rent_majorated: Number(lease.reference_rent_majorated ?? 0),
    rent_complement: Number(lease.rent_complement ?? 0),
    rent_complement_justification: lease.rent_complement_justification ?? '',
    previous_tenant_departure_date: lease.previous_tenant_departure_date ?? '',
    previous_tenant_last_rent: Number(lease.previous_tenant_last_rent ?? 0),
    professional_mandate: lease.professional_mandate,
    agency_fee_tenant: Number(lease.agency_fee_tenant ?? 0),
    agency_fee_landlord: Number(lease.agency_fee_landlord ?? 0),
    custom_clauses: lease.custom_clauses ?? '',
    inventory_date: lease.inventory_date || '',
    private_room_label: lease.private_room_label || '',
    shared_areas_text: lease.shared_areas_text || '',
    furniture_set_ids: [...lease.furniture_set_ids],
    furniture_inventory: lease.furniture_inventory || '',
    erp: lease.erp || '',
    home_insurance: lease.home_insurance || '',
    legal_notice_provided: lease.legal_notice_provided,
    annex_entry_inventory_provided: lease.annex_entry_inventory_provided,
    annex_furniture_inventory_provided: lease.annex_furniture_inventory_provided,
    annex_dpe_provided: lease.annex_dpe_provided,
    annex_erp_provided: lease.annex_erp_provided,
    annex_home_insurance_provided: lease.annex_home_insurance_provided,
    identifiant_fiscal: lease.identifiant_fiscal || '',
    habitat_type: lease.habitat_type || 'collectif',
    regime_juridique: lease.regime_juridique || 'copropriete',
    construction_period: lease.construction_period || '1989_2005',
    electrical_installation_over_15y: lease.electrical_installation_over_15y ?? false,
    gas_installation_over_15y: lease.gas_installation_over_15y ?? false,
    in_risk_zone: lease.in_risk_zone ?? false,
    annex_lead_provided: lease.annex_lead_provided ?? false,
    annex_electrical_provided: lease.annex_electrical_provided ?? false,
    annex_gas_provided: lease.annex_gas_provided ?? false,
    annex_risk_provided: lease.annex_risk_provided ?? false,
  }
}

const leaseData = computed<LeaseData | null>(() => {
  if (!property.value || !primaryTenant.value) return null

  // Determine landlord based on property ownership
  let landlordData
  if (property.value.organization_id && organizationsStore.currentOrganization) {
    // Use organization as landlord
    const org = organizationsStore.currentOrganization
    const ownerMember = org.members.find(member => member.role === 'owner')
    landlordData = {
      name: org.name,
      address: org.address,
      addressLabel: 'Siège social' as const,
      legalForm: org.legal_form,
      siret: org.siret,
      legalRepresentative: ownerMember?.user_name || authStore.user?.name,
      birthDate: undefined,
      birthPlace: undefined
    }
  } else if (authStore.user) {
    // Use user as landlord
    landlordData = {
      name: authStore.user.name,
      address: authStore.user.address,
      addressLabel: 'Adresse' as const,
      birthDate: authStore.user.birth_date,
      birthPlace: authStore.user.birth_place
    }
  } else {
    return null
  }

  return {
    landlord: landlordData,
    tenant: {
      name: primaryTenant.value.name,
      address: primaryTenant.value.address || '',
      birthDate: primaryTenant.value.birth_date,
      birthPlace: primaryTenant.value.birth_place
    },
    property: {
      address: property.value.address,
      type: property.value.furnished ? 'furnished' : 'unfurnished',
      surface: Number(property.value.surface_area) || 0,
      rooms: property.value.rooms || 0,
      description: property.value.description
    },
    terms: {
      startDate: formData.value.start_date || '',
      duration: formData.value.duration_months,
      leaseKind: formData.value.lease_kind,
      isColocation: formData.value.is_colocation,
      tenantCount: formData.value.tenant_ids.length,
      monthlyRent: formData.value.monthly_rent,
      charges: formData.value.charges,
      deposit: formData.value.deposit,
      rentRevision: formData.value.rent_revision,
      annualChargesRegularization: false,
      inventoryDate: formData.value.inventory_date || undefined,
      privateRoomLabel: formData.value.private_room_label || undefined,
      sharedAreasText: formData.value.shared_areas_text || undefined,
      rentControlled: formData.value.rent_controlled,
      referenceRent: formData.value.rent_controlled ? formData.value.reference_rent : undefined,
      referenceRentMajorated: formData.value.rent_controlled ? formData.value.reference_rent_majorated : undefined,
      rentComplement: formData.value.rent_controlled ? formData.value.rent_complement : undefined,
      rentComplementJustification: formData.value.rent_complement_justification || undefined,
    },
    annexes: {
      furnitureInventory: formData.value.furniture_inventory || undefined,
      furnitureSets: selectedFurnitureSets.value.map(set => ({
        name: set.name,
        items: set.items.map(item => ({
          category: item.category,
          name: item.name,
          quantity: item.quantity,
          itemCondition: item.item_condition,
        }))
      })),
      dpe: formData.value.dpe_class ? `Classe ${formData.value.dpe_class}` : undefined,
      erp: formData.value.erp || undefined,
      homeInsurance: formData.value.home_insurance || undefined,
      legalNoticeProvided: formData.value.legal_notice_provided,
      professionalMandate: formData.value.professional_mandate,
      agencyFeeTenant: formData.value.professional_mandate ? formData.value.agency_fee_tenant : undefined,
      agencyFeeLandlord: formData.value.professional_mandate ? formData.value.agency_fee_landlord : undefined,
      customClauses: formData.value.custom_clauses || undefined,
    }
  }
})

onMounted(async () => {
  loading.value = true
  error.value = null

  try {
    await Promise.all([
      propertiesStore.fetchProperty(propertyId.value),
      tenantsStore.fetchTenants(),
      authStore.user ? Promise.resolve() : authStore.fetchCurrentUser()
    ])

    if (!property.value) {
      error.value = 'Propriété non trouvée'
      return
    }

    formData.value.habitable_surface = Number(property.value.surface_area) || 0
    formData.value.main_room_count = property.value.rooms || 1

    // Fetch organization if property belongs to one
    if (property.value.organization_id) {
      await organizationsStore.fetchOrganizationById(property.value.organization_id)
    }

    furnitureSets.value = await propertiesStore.listFurnitureSets(propertyId.value)

    await leasesStore.fetchLeases(propertyId.value)

    const leaseIdFromQuery = typeof route.query.leaseId === 'string'
      ? route.query.leaseId
      : null

    let existingLease: Lease | undefined
    if (leaseIdFromQuery) {
      existingLease = leasesStore.getLeaseById(leaseIdFromQuery)
      if (!existingLease) {
        existingLease = await leasesStore.fetchLease(leaseIdFromQuery)
      }
    }

    if (existingLease) {
      generatedLeaseId.value = existingLease.id
      applyLeaseToForm(existingLease)
      await loadSelectedFurnitureSet()
      await router.replace({
        path: route.path,
        query: { ...route.query, leaseId: existingLease.id }
      })
    }
  } catch (err: any) {
    error.value = err.message || 'Erreur lors du chargement'
  } finally {
    loading.value = false
  }
})

async function loadSelectedFurnitureSet() {
  if (formData.value.furniture_set_ids.length === 0) {
    selectedFurnitureSets.value = []
    return
  }

  selectedFurnitureSets.value = await Promise.all(
    formData.value.furniture_set_ids.map(setId =>
      propertiesStore.getFurnitureSet(propertyId.value, setId),
    )
  )
}

async function generateLease() {
  // Validate required fields
  if (formData.value.tenant_ids.length === 0 || !formData.value.start_date ||
      formData.value.monthly_rent <= 0 || formData.value.charges < 0 || 
      formData.value.deposit < 0) {
    error.value = 'Veuillez remplir tous les champs obligatoires'
    return
  }

  if (formData.value.is_colocation && formData.value.tenant_ids.length < 2) {
    error.value = 'Une colocation requiert au moins 2 locataires'
    return
  }

  if (!formData.value.is_colocation && formData.value.tenant_ids.length > 1) {
    error.value = "Plusieurs locataires nécessitent d'activer la colocation"
    return
  }

  if (formData.value.habitable_surface <= 0 || formData.value.main_room_count <= 0) {
    error.value = 'La surface habitable et le nombre de pieces principales sont obligatoires'
    return
  }

  try {
    loading.value = true
    error.value = null

    await loadSelectedFurnitureSet()

    const payload = {
      property_id: propertyId.value,
      tenant_ids: formData.value.tenant_ids,
      start_date: formData.value.start_date || '',
      duration_months: formData.value.duration_months,
      lease_kind: formData.value.lease_kind,
      is_colocation: formData.value.is_colocation,
      destination: formData.value.destination,
      monthly_rent: formData.value.monthly_rent,
      charges: formData.value.charges,
      deposit: formData.value.deposit,
      rent_revision: formData.value.rent_revision,
      habitable_surface: formData.value.habitable_surface,
      main_room_count: formData.value.main_room_count,
      heating_mode: formData.value.heating_mode,
      hot_water_mode: formData.value.hot_water_mode,
      dpe_class: formData.value.dpe_class,
      is_dom_tom: formData.value.is_dom_tom,
      energy_cost_annual: formData.value.energy_cost_annual || undefined,
      energy_cost_year: formData.value.energy_cost_year || undefined,
      rent_payment_frequency: formData.value.rent_payment_frequency,
      rent_payment_timing: formData.value.rent_payment_timing,
      rent_payment_period: formData.value.rent_payment_period || undefined,
      rent_controlled: formData.value.rent_controlled,
      reference_rent: formData.value.rent_controlled ? formData.value.reference_rent : undefined,
      reference_rent_majorated: formData.value.rent_controlled ? formData.value.reference_rent_majorated : undefined,
      rent_complement: formData.value.rent_controlled ? formData.value.rent_complement : undefined,
      rent_complement_justification: formData.value.rent_complement_justification || undefined,
      previous_tenant_departure_date: formData.value.previous_tenant_departure_date || undefined,
      previous_tenant_last_rent: formData.value.previous_tenant_last_rent > 0 ? formData.value.previous_tenant_last_rent : undefined,
      professional_mandate: formData.value.professional_mandate,
      agency_fee_tenant: formData.value.professional_mandate ? formData.value.agency_fee_tenant : undefined,
      agency_fee_landlord: formData.value.professional_mandate ? formData.value.agency_fee_landlord : undefined,
      custom_clauses: formData.value.custom_clauses || undefined,
      annual_charges_regularization: false,
      inventory_date: formData.value.inventory_date || undefined,
      private_room_label: formData.value.private_room_label || undefined,
      shared_areas_text: formData.value.shared_areas_text || undefined,
      furniture_set_ids: formData.value.furniture_set_ids,
      furniture_inventory: formData.value.furniture_inventory || undefined,
      dpe: formData.value.dpe_class ? `Classe ${formData.value.dpe_class}` : undefined,
      erp: formData.value.erp || undefined,
      home_insurance: formData.value.home_insurance || undefined,
      legal_notice_provided: formData.value.legal_notice_provided,
      annex_entry_inventory_provided: formData.value.annex_entry_inventory_provided,
      annex_furniture_inventory_provided: property.value?.furnished ? formData.value.annex_furniture_inventory_provided : true,
      annex_dpe_provided: formData.value.annex_dpe_provided,
      annex_erp_provided: formData.value.annex_erp_provided,
      annex_home_insurance_provided: formData.value.annex_home_insurance_provided,
      identifiant_fiscal: formData.value.identifiant_fiscal || undefined,
      habitat_type: formData.value.habitat_type,
      regime_juridique: formData.value.regime_juridique,
      construction_period: formData.value.construction_period,
      electrical_installation_over_15y: formData.value.electrical_installation_over_15y,
      gas_installation_over_15y: formData.value.gas_installation_over_15y,
      in_risk_zone: formData.value.in_risk_zone,
      annex_lead_provided: formData.value.annex_lead_provided,
      annex_electrical_provided: formData.value.annex_electrical_provided,
      annex_gas_provided: formData.value.annex_gas_provided,
      annex_risk_provided: formData.value.annex_risk_provided,
    }

    const isCreating = !generatedLeaseId.value
    const lease = isCreating
      ? await leasesStore.createLease(payload)
      : await leasesStore.updateLease(generatedLeaseId.value as string, payload)

    generatedLeaseId.value = lease.id
    generatedComplianceStatus.value = lease.compliance_status ?? null
    isNewlyCreated.value = isCreating

    if (!isCreating) {
      await receiptsStore.regenerateForLease(lease.id, true)
    }
    
    // Reload leases for this property to ensure PropertyDetail has fresh data
    await leasesStore.fetchLeases(propertyId.value)
    
    await router.replace({
      path: route.path,
      query: { ...route.query, leaseId: lease.id }
    })

    console.log('Lease created/updated:', lease)
    error.value = null

    // Show PDF preview
    showPreview.value = true
  } catch (err: any) {
    error.value = err.message || 'Erreur lors de la génération'
  } finally {
    loading.value = false
  }
}

function back() {
  showPreview.value = false
  isNewlyCreated.value = false
  router.push({
    path: `/properties/${propertyId.value}`,
    query: { tab: 'leases' }
  })
}
</script>

<template>
  <div v-if="loading" class="loading c-state c-state--loading">Chargement...</div>

  <div v-else-if="error" class="error-state c-state c-state--error">
    <p>❌ {{ error }}</p>
    <button @click="router.push({ path: `/properties/${propertyId}`, query: { tab: 'leases' } })" class="btn-secondary c-button c-button--secondary">
      Retour
    </button>
  </div>

  <div v-else-if="!showPreview" class="generate-lease">
    <div class="header l-page__header">
      <div>
        <p class="context-label">Propriété / Baux</p>
        <h1>{{ isEditMode ? 'Modifier le bail' : 'Créer un nouveau bail' }}</h1>
      </div>
      <button @click="back" class="btn-secondary c-button c-button--secondary">← Retour à la liste des baux</button>
    </div>

    <p v-if="isEditMode" class="edit-mode-note">
      Mode édition actif: les modifications mettront à jour le bail déjà créé au lieu d'en créer un nouveau.
    </p>

    <div class="card">
      <h2>Informations de la propriété</h2>
      <div class="info-section">
        <div><strong>Adresse:</strong> {{ property?.address }}</div>
        <div><strong>Type:</strong> {{ property?.property_type }}</div>
        <div><strong>Meublé:</strong> {{ property?.furnished ? 'Oui' : 'Non' }}</div>
      </div>

      <form @submit.prevent="generateLease" class="lease-form">
        <h3 class="form-section-title">Parties et dates</h3>
        <div class="form-group">
          <label>{{ formData.is_colocation ? 'Colocataires *' : 'Locataire *' }}</label>
          <div class="tenant-multi-list">
            <label v-for="tenant in allTenants" :key="tenant.id" class="tenant-option">
              <input
                type="checkbox"
                :value="tenant.id"
                v-model="formData.tenant_ids"
              />
              <span>{{ tenant.name }}</span>
              <span v-if="formData.tenant_ids[0] === tenant.id" class="primary-badge">principal</span>
            </label>
          </div>
          <small class="hint-text">
            {{ formData.is_colocation
              ? 'Sélectionnez au moins deux colocataires. Le premier sélectionné est le locataire principal.'
              : 'Sélectionnez le locataire.' }}
          </small>
          <router-link to="/tenants" class="link">+ Ajouter un nouveau locataire</router-link>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="startDate">Date de début *</label>
            <input type="date" id="startDate" v-model="formData.start_date" required />
          </div>

          <div class="form-group">
            <label for="duration">Durée (mois) *</label>
            <input type="number" id="duration" v-model="formData.duration_months" min="1" required />
          </div>

          <div class="form-group">
            <label for="leaseKind">Type de bail *</label>
            <select id="leaseKind" v-model="formData.lease_kind">
              <option value="standard">Meublé standard (12 mois+)</option>
              <option value="student">Meublé étudiant (9 mois)</option>
            </select>
          </div>
        </div>

        <h3 class="form-section-title">Logement</h3>
        <div class="form-row">
          <div class="form-group">
            <label for="surface">Surface habitable (m2) *</label>
            <input type="number" id="surface" v-model="formData.habitable_surface" min="0.01" step="0.01" required />
          </div>
          <div class="form-group">
            <label for="rooms">Pieces principales *</label>
            <input type="number" id="rooms" v-model="formData.main_room_count" min="1" required />
          </div>
        </div>

        <div v-if="!formData.is_dom_tom" class="form-group">
          <label for="ifl">Identifiant fiscal du logement (IFL) *</label>
          <input type="text" id="ifl" v-model="formData.identifiant_fiscal" placeholder="Ex: 1234567890ABC" />
          <small class="hint-text">Obligatoire (sauf DOM-TOM). Figure sur l'avis de taxe foncière.</small>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="habitatType">Type d'habitat *</label>
            <select id="habitatType" v-model="formData.habitat_type">
              <option value="collectif">Collectif</option>
              <option value="individuel">Individuel</option>
            </select>
          </div>
          <div class="form-group">
            <label for="regimeJuridique">Régime juridique *</label>
            <select id="regimeJuridique" v-model="formData.regime_juridique">
              <option value="monopropriete">Monopropriété</option>
              <option value="copropriete">Copropriété</option>
            </select>
          </div>
          <div class="form-group">
            <label for="constructionPeriod">Période de construction *</label>
            <select id="constructionPeriod" v-model="formData.construction_period">
              <option value="avant_1949">Avant 1949</option>
              <option value="1949_1974">De 1949 à 1974</option>
              <option value="1975_1989">De 1975 à 1989</option>
              <option value="1989_2005">De 1989 à 2005</option>
              <option value="depuis_2005">Depuis 2005</option>
            </select>
          </div>
        </div>

        <h3 class="form-section-title">Conditions financières</h3>
        <div class="form-row">
          <div class="form-group">
            <label for="rent">Loyer mensuel (€) *</label>
            <input type="number" id="rent" v-model="formData.monthly_rent" min="0" step="0.01" required />
          </div>

          <div class="form-group">
            <label for="charges">Charges (€) *</label>
            <input type="number" id="charges" v-model="formData.charges" min="0" step="0.01" required />
          </div>
        </div>

        <div class="form-group">
          <label for="deposit">Dépôt de garantie (€) *</label>
          <input type="number" id="deposit" v-model="formData.deposit" min="0" step="0.01" required />
          <small class="hint-text">
            Saisissez 0 si aucun dépôt de garantie n'est exigé.
          </small>
        </div>

        <div class="form-group">
          <label for="inventoryDate">Date de l'état des lieux</label>
          <input type="date" id="inventoryDate" v-model="formData.inventory_date" />
        </div>

        <div class="form-group checkbox">
          <label>
            <input type="checkbox" v-model="formData.rent_revision" />
            Clause de révision du loyer
          </label>
        </div>

        <h3 class="form-section-title">Colocation</h3>
        <div class="form-row">
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.is_colocation" />
              Colocation
            </label>
          </div>
          <div class="form-group">
            <label>Nombre de {{ formData.is_colocation ? 'colocataires' : 'locataires' }}</label>
            <p class="derived-count">
              {{ formData.tenant_ids.length }}
              <span v-if="formData.is_colocation && formData.tenant_ids.length < 2" class="count-warning">
                (au moins 2 requis)
              </span>
              <span v-else-if="!formData.is_colocation && formData.tenant_ids.length > 1" class="count-warning">
                (un seul locataire hors colocation)
              </span>
            </p>
            <small class="hint-text">Défini par les locataires sélectionnés ci-dessus.</small>
          </div>
        </div>

        <div class="form-row" v-if="formData.is_colocation">
          <div class="form-group">
            <label for="privateRoom">Chambre privative *</label>
            <input
              type="text"
              id="privateRoom"
              v-model="formData.private_room_label"
              placeholder="Ex: 4"
            />
          </div>

          <div class="form-group">
            <label for="sharedAreas">Parties communes partagées</label>
            <input
              type="text"
              id="sharedAreas"
              v-model="formData.shared_areas_text"
              placeholder="Ex: salon, cuisine, salle à manger, buanderie"
            />
          </div>
        </div>

        <h3 class="form-section-title">Diagnostic énergétique</h3>
        <div class="form-row">
          <div class="form-group">
            <label for="dpeClass">Classe DPE *</label>
            <select id="dpeClass" v-model="formData.dpe_class">
              <option value="A">A</option>
              <option value="B">B</option>
              <option value="C">C</option>
              <option value="D">D</option>
              <option value="E">E</option>
              <option value="F">F</option>
              <option value="G">G</option>
            </select>
          </div>
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.is_dom_tom" />
              Logement en DOM-TOM
            </label>
          </div>
        </div>

        <h3 class="form-section-title">Zone encadrée des loyers</h3>
        <div class="form-row">
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.rent_controlled" />
              Zone encadree des loyers
            </label>
          </div>
        </div>

        <div class="form-row" v-if="formData.rent_controlled">
          <div class="form-group">
            <label for="referenceRent">Loyer de reference (EUR/m2) *</label>
            <input type="number" id="referenceRent" v-model="formData.reference_rent" min="0" step="0.01" />
          </div>
          <div class="form-group">
            <label for="referenceRentMajorated">Loyer de reference majore (EUR/m2) *</label>
            <input type="number" id="referenceRentMajorated" v-model="formData.reference_rent_majorated" min="0" step="0.01" />
          </div>
        </div>

        <div class="form-row" v-if="formData.rent_controlled">
          <div class="form-group">
            <label for="rentComplement">Complement de loyer (EUR)</label>
            <input type="number" id="rentComplement" v-model="formData.rent_complement" min="0" step="0.01" />
          </div>
          <div class="form-group">
            <label for="rentComplementJustification">Justification complement</label>
            <input type="text" id="rentComplementJustification" v-model="formData.rent_complement_justification" />
          </div>
        </div>

        <h3 class="form-section-title">Mandataire professionnel</h3>
        <div class="form-row">
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.professional_mandate" />
              Mandataire professionnel
            </label>
          </div>
        </div>

        <div class="form-row" v-if="formData.professional_mandate">
          <div class="form-group">
            <label for="agencyFeeTenant">Honoraires locataire (EUR) *</label>
            <input type="number" id="agencyFeeTenant" v-model="formData.agency_fee_tenant" min="0" step="0.01" />
          </div>
          <div class="form-group">
            <label for="agencyFeeLandlord">Honoraires bailleur (EUR) *</label>
            <input type="number" id="agencyFeeLandlord" v-model="formData.agency_fee_landlord" min="0" step="0.01" />
          </div>
        </div>

        <h3 class="form-section-title">Clauses particulières</h3>
        <div class="form-group">
          <label for="customClauses">Clauses particulieres (controle legal applique)</label>
          <textarea id="customClauses" v-model="formData.custom_clauses" rows="3" />
        </div>

        <div class="form-group legal-note">
          Charges au forfait: aucune régularisation annuelle n'a lieu.
        </div>

        <h3>Annexes et mentions légales</h3>

        <div v-if="property?.furnished" class="form-group">
          <label>Sets de mobilier (un ou plusieurs)</label>
          <div class="furniture-set-list">
            <label v-for="set in furnitureSets" :key="set.id" class="furniture-set-option">
              <input
                :value="set.id"
                type="checkbox"
                v-model="formData.furniture_set_ids"
                @change="loadSelectedFurnitureSet"
              />
              <span>{{ set.name }}</span>
            </label>
          </div>
          <small class="hint-text">
            Vous pouvez créer/modifier des sets de mobilier dans la fiche de la propriété.
          </small>
        </div>

        <div v-if="property?.furnished" class="form-group">
          <label for="furnitureInventory">Notes complémentaires sur le mobilier</label>
          <textarea
            id="furnitureInventory"
            v-model="formData.furniture_inventory"
            rows="3"
            placeholder="Précisions libres en complément des sets de mobilier structurés ci-dessus."
          />
        </div>

        <div class="form-group">
          <label for="erp">État des risques (ERP)</label>
          <input
            type="text"
            id="erp"
            v-model="formData.erp"
            placeholder="Ex: ERP daté du 01/05/2026"
          />
        </div>

        <div class="form-group">
          <label for="homeInsurance">Assurance habitation mentionnée</label>
          <input
            type="text"
            id="homeInsurance"
            v-model="formData.home_insurance"
            placeholder="Ex: Attestation à fournir à l'entrée puis annuellement"
          />
        </div>

        <div class="form-group checkbox">
          <label>
            <input type="checkbox" v-model="formData.legal_notice_provided" />
            Notice d'information légale locataire remise
          </label>
        </div>

        <div class="form-row">
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.annex_entry_inventory_provided" />
              Annexe état des lieux d'entrée fournie
            </label>
          </div>
          <div class="form-group checkbox" v-if="property?.furnished">
            <label>
              <input type="checkbox" v-model="formData.annex_furniture_inventory_provided" />
              Annexe inventaire mobilier fournie
            </label>
          </div>
        </div>

        <div class="form-row">
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.annex_dpe_provided" />
              Annexe DPE fournie
            </label>
          </div>
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.annex_erp_provided" />
              Annexe ERP fournie
            </label>
          </div>
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.annex_home_insurance_provided" />
              Attestation assurance fournie
            </label>
          </div>
        </div>

        <h3 class="form-section-title">Diagnostics conditionnels</h3>
        <p class="hint-text">
          Cochez les situations applicables ; les annexes correspondantes deviennent obligatoires.
        </p>

        <div v-if="formData.construction_period === 'avant_1949'" class="form-group checkbox">
          <label>
            <input type="checkbox" v-model="formData.annex_lead_provided" />
            Constat de risque d'exposition au plomb (Crep) fourni — obligatoire (construction avant 1949)
          </label>
        </div>

        <div class="form-row">
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.electrical_installation_over_15y" />
              Installation électrique de plus de 15 ans
            </label>
          </div>
          <div class="form-group checkbox" v-if="formData.electrical_installation_over_15y">
            <label>
              <input type="checkbox" v-model="formData.annex_electrical_provided" />
              Diagnostic électricité fourni
            </label>
          </div>
        </div>

        <div class="form-row">
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.gas_installation_over_15y" />
              Installation gaz de plus de 15 ans
            </label>
          </div>
          <div class="form-group checkbox" v-if="formData.gas_installation_over_15y">
            <label>
              <input type="checkbox" v-model="formData.annex_gas_provided" />
              Diagnostic gaz fourni
            </label>
          </div>
        </div>

        <div class="form-row">
          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.in_risk_zone" />
              Logement en zone à risques (ERNT)
            </label>
          </div>
          <div class="form-group checkbox" v-if="formData.in_risk_zone">
            <label>
              <input type="checkbox" v-model="formData.annex_risk_provided" />
              État des risques (ERNT) fourni
            </label>
          </div>
        </div>

        <div v-if="complianceWarnings.length > 0" class="compliance-warning-box">
          <strong>Points de conformite a corriger avant generation:</strong>
          <ul>
            <li v-for="warning in complianceWarnings" :key="warning">{{ warning }}</li>
          </ul>
        </div>

        <button type="submit" class="btn-primary c-button c-button--primary" :disabled="formData.tenant_ids.length === 0 || loading">
          {{ isEditMode ? '💾 Mettre à jour le bail et régénérer le PDF' : '📄 Créer le bail et générer le PDF' }}
        </button>
      </form>
    </div>
  </div>

  <LeasePreview 
    v-else-if="leaseData" 
    :property-id="propertyId"
    :lease-id="generatedLeaseId ?? undefined"
    :compliance-status="generatedComplianceStatus ?? undefined"
    :is-newly-created="isNewlyCreated"
    @back="back"
  />
</template>

<style scoped>
.generate-lease {
  max-width: 800px;
  margin: 0 auto;
}

.edit-mode-note {
  margin: 0 0 1rem;
  padding: 0.75rem 1rem;
  border-radius: 8px;
  background: #e8f5e9;
  color: #1b5e20;
  font-weight: 500;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.header h1 {
  margin: 0;
}

.context-label {
  margin: 0 0 0.35rem;
  color: #667eea;
  font-size: 0.88rem;
  font-weight: 700;
  letter-spacing: 0.02em;
  text-transform: uppercase;
}

.card {
  background: var(--color-surface);
  padding: 2rem;
  border-radius: 12px;
  box-shadow: var(--shadow-md);
}

.card h2 {
  margin-top: 0;
  color: var(--color-text-strong);
}

.info-section {
  background: var(--color-surface-muted);
  padding: 1.5rem;
  border-radius: 8px;
  margin-bottom: 2rem;
  display: grid;
  gap: 0.75rem;
}

.lease-form {
  display: grid;
  gap: 1.5rem;
}

.form-section-title {
  margin: 0.5rem 0 0;
  padding-bottom: 0.4rem;
  border-bottom: 1px solid #e5e7eb;
  font-size: 1.05rem;
  font-weight: 600;
  color: #374151;
}

.tenant-multi-list {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 0.6rem 0.75rem;
}

.tenant-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.primary-badge {
  font-size: 0.75rem;
  font-weight: 600;
  color: #15803d;
  background: #dcfce7;
  border-radius: 999px;
  padding: 0.05rem 0.5rem;
}

.derived-count {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
}

.count-warning {
  font-size: 0.85rem;
  font-weight: 500;
  color: #b45309;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group.checkbox {
  flex-direction: row;
  align-items: center;
}

.form-group.checkbox label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.form-group label {
  font-weight: 600;
  color: var(--color-text-strong);
}

.form-group input,
.form-group select {
  padding: 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  font-size: 1rem;
  background: var(--color-surface);
  color: var(--color-text);
}

.form-group input[type="checkbox"] {
  width: auto;
  cursor: pointer;
}

.link {
  color: #667eea;
  font-size: 0.9rem;
  text-decoration: none;
}

.link:hover {
  text-decoration: underline;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 1rem 2rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  transition: transform 0.2s;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: #f5f5f5;
  color: #333;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  transition: background 0.2s;
}

.btn-secondary:hover {
  background: #e0e0e0;
}

.compliance-warning-box {
  border: 1px solid #e6a700;
  background: #fff8e6;
  color: #704d00;
  border-radius: 8px;
  padding: 0.75rem 1rem;
}

.compliance-warning-box ul {
  margin: 0.5rem 0 0;
  padding-left: 1.2rem;
}

.error-state, .loading {
  text-align: center;
  padding: 3rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }
}
</style>
